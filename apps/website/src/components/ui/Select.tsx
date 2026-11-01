import type { SelectHTMLAttributes } from "react";

type SelectProps = SelectHTMLAttributes<HTMLSelectElement> & {
  label: string;
  options: {
    label: string;
    value: string;
  }[];
  placeholder?: string;
};

export function Select({
  label,
  id,
  required,
  options,
  placeholder = "Selecione uma opção",
  className = "",
  ...props
}: SelectProps) {
  return (
    <div className="w-full">
      <label htmlFor={id} className="mb-3 block text-sm text-white/65">
        {label} {required && "*"}
      </label>

      <select
        id={id}
        defaultValue=""
        className={`w-full border-b border-white/20 bg-transparent px-0 py-3 text-sm text-white/70 outline-none transition focus:border-white ${className}`}
        {...props}
        required={required}
      >
        <option value="" disabled className="bg-(--color-light-black)">
          {placeholder}
        </option>

        {options.map(({ label, value }) => (
          <option
            key={value}
            value={value}
            className="bg-(--color-light-black)"
          >
            {label}
          </option>
        ))}
      </select>
    </div>
  );
}
