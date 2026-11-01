import type { Mail } from "lucide-react";

type ContactInfoItemProps = {
  icon: typeof Mail;
  title: string;
  children: React.ReactNode;
};

export function ContactInfoItem({
  icon: Icon,
  title,
  children,
}: ContactInfoItemProps) {
  return (
    <div className="flex gap-5">
      <Icon className="mt-1 shrink-0 text-white/55" size={20} />

      <div>
        <h4 className="text-sm font-medium">{title}</h4>
        <div className="mt-2 text-sm text-white/50">{children}</div>
      </div>
    </div>
  );
}
