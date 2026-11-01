import type { TextareaHTMLAttributes } from "react";

type TextAreaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & {
  label: string;
};

export function TextArea({
  label,
  id,
  required,
  className = "",
  ...props
}: TextAreaProps) {
  return (
    <div className="w-full">
      <label htmlFor={id} className="mb-3 block text-sm text-white/65">
        {label} {required && "*"}
      </label>

      <textarea
        id={id}
        className={`w-full resize-none border-b border-white/20 bg-transparent px-0 py-3 text-sm outline-none transition placeholder:text-white/30 focus:border-white ${className}`}
        {...props}
        required={required}
      />
    </div>
  );
}
