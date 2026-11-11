type Props = {
  title: string;
  description: string;
};

export function DashboardEmptyState({ title, description }: Props) {
  return (
    <div className="flex flex-col items-center justify-center px-5 py-10 text-center">
      <p className="text-sm font-medium">{title}</p>

      <p className="mt-1 max-w-xs text-xs text-muted-foreground">
        {description}
      </p>
    </div>
  );
}
