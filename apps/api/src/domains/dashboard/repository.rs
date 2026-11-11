use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    domains::dashboard::dtos::{
        DashboardProjectDto, DashboardResponseDto, DashboardStatisticsDto, DashboardTaskDto,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn get_dashboard(
        pool: &PgPool,
        user_id: Uuid,
        is_client: bool,
    ) -> AppResult<DashboardResponseDto> {
        let mut statistics_query = QueryBuilder::<Postgres>::new(
            r#"
                SELECT
                    project_stats.active_projects,
                    project_stats.completed_projects,
                    task_stats.pending_tasks,
                    task_stats.overdue_tasks
                FROM (
                    SELECT
                        COUNT(*) FILTER (
                            WHERE p.status IN ('planning', 'in_progress', 'paused')
                        ) AS active_projects,
                        COUNT(*) FILTER (
                            WHERE p.status = 'completed'
                        ) AS completed_projects
                    FROM projects p
                    WHERE TRUE
            "#,
        );
        if is_client {
            statistics_query
                .push(" AND p.client_id = (SELECT client_id FROM users WHERE id = ")
                .push_bind(user_id)
                .push(")");
        }
        statistics_query.push(
            r#"
                ) AS project_stats
                CROSS JOIN (
                    SELECT
                        COUNT(*) AS pending_tasks,
                        COUNT(*) FILTER (
                            WHERE t.due_date < CURRENT_DATE
                        ) AS overdue_tasks
                    FROM tasks t
                    JOIN stages s ON s.id = t.stage_id
                    JOIN projects p ON p.id = s.project_id
                    WHERE t.status IN ('pending', 'in_progress', 'blocked')
                      AND p.status NOT IN ('cancelled', 'completed')
            "#,
        );
        if is_client {
            statistics_query
                .push(" AND p.client_id = (SELECT client_id FROM users WHERE id = ")
                .push_bind(user_id)
                .push(")");
        } else {
            statistics_query
                .push(" AND t.assigned_to = ")
                .push_bind(user_id);
        }
        let statistics = statistics_query
            .push(") AS task_stats")
            .build_query_as::<DashboardStatisticsDto>()
            .fetch_one(pool)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao carregar estatísticas do dashboard.".to_string())
            })?;

        let mut projects_query = QueryBuilder::<Postgres>::new(
            r#"
                SELECT
                    p.id,
                    p.name,
                    p.status,
                    p.city,
                    p.state,
                    p.estimated_end_date,
                    progress.value::int AS progress,
                    CASE
                        WHEN cover.id IS NULL THEN NULL
                        ELSE '/project-images/' || cover.id::text || '/file'
                    END AS cover_image_url
                FROM (
                    SELECT
                        p.id,
                        p.name,
                        p.status::text AS status,
                        p.city,
                        p.state,
                        p.estimated_end_date,
                        CASE p.status
                            WHEN 'in_progress' THEN 0
                            WHEN 'planning' THEN 1
                            WHEN 'paused' THEN 2
                            ELSE 3
                        END AS status_order,
                        p.updated_at
                    FROM projects p
                    WHERE p.status IN ('planning', 'in_progress', 'paused')
            "#,
        );
        if is_client {
            projects_query
                .push(" AND p.client_id = (SELECT client_id FROM users WHERE id = ")
                .push_bind(user_id)
                .push(")");
        }
        projects_query.push(
            r#"
                    ORDER BY status_order, p.updated_at DESC
                    LIMIT 6
                ) AS p
                LEFT JOIN LATERAL (
                    SELECT ROUND(
                        100 * SUM(s.weight) FILTER (WHERE s.status = 'completed')
                        / NULLIF(SUM(s.weight) FILTER (WHERE s.status <> 'cancelled'), 0)
                    ) AS value
                    FROM stages s
                    WHERE s.project_id = p.id
                ) AS progress ON TRUE
                LEFT JOIN project_images AS cover
                    ON cover.project_id = p.id
                   AND cover."order" = 0
                ORDER BY p.status_order, p.updated_at DESC
            "#,
        );
        let projects = projects_query
            .build_query_as::<DashboardProjectDto>()
            .fetch_all(pool)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao carregar projetos do dashboard.".to_string())
            })?;

        let mut tasks_query = QueryBuilder::<Postgres>::new(
            r#"
                SELECT
                    t.id,
                    t.title,
                    t.status::text AS status,
                    t.due_date,
                    COALESCE(t.due_date < CURRENT_DATE, false) AS is_overdue,
                    s.name AS stage_name,
                    p.name AS project_name
                FROM tasks t
                JOIN stages s ON s.id = t.stage_id
                JOIN projects p ON p.id = s.project_id
                WHERE t.status IN ('pending', 'in_progress', 'blocked')
                  AND p.status NOT IN ('cancelled', 'completed')
            "#,
        );
        if is_client {
            tasks_query
                .push(" AND p.client_id = (SELECT client_id FROM users WHERE id = ")
                .push_bind(user_id)
                .push(")");
        } else {
            tasks_query.push(" AND t.assigned_to = ").push_bind(user_id);
        }
        let tasks = tasks_query
            .push(" ORDER BY t.due_date ASC NULLS LAST, t.updated_at DESC LIMIT 8")
            .build_query_as::<DashboardTaskDto>()
            .fetch_all(pool)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao carregar tarefas do dashboard.".to_string())
            })?;

        Ok(DashboardResponseDto {
            statistics,
            projects,
            tasks,
        })
    }
}
