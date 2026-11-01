import type { InputHTMLAttributes } from "react";

type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  label: string;
};

export function Input({
  label,
  id,
  required,
  className = "",
  ...props
}: InputProps) {
  return (
    <div className="w-full">
      <label htmlFor={id} className="mb-3 block text-sm text-white/65">
        {label} {required && "*"}
      </label>

      <input
        id={id}
        className={`w-full border-b border-white/20 bg-transparent px-0 py-3 text-sm outline-none transition placeholder:text-white/30 focus:border-white ${className}`}
        {...props}
        required={required}
      />
    </div>
  );
}
