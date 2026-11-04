type Props = {
  message?: string;
};

export function Error({ message }: Props) {
  if (!message) return null;

  return (
    <div
      role="alert"
      className="rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive"
    >
      {message}
    </div>
  );
}
