import { Loader } from "@/components";

export function LoadingPage() {
  return (
    <div className="flex min-h-dvh min-w-full items-center justify-center bg-background">
      <Loader />
    </div>
  );
}
