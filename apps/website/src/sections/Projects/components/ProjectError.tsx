type ProjectsErrorProps = {
  message: string;
  onRetry: () => void;
};

export function ProjectsError({ message, onRetry }: ProjectsErrorProps) {
  return (
    <div className="flex min-h-64 flex-col items-center justify-center text-center">
      <p className="text-lg font-medium">
        Não foi possível carregar os projetos.
      </p>

      <p className="mt-2 text-sm text-gray-500">{message}</p>

      <button
        type="button"
        onClick={onRetry}
        className="mt-4 border-b border-gray-400 pb-1 text-sm"
      >
        Tentar novamente
      </button>
    </div>
  );
}
