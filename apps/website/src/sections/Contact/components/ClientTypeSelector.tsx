import type { ClientType } from "../../../types";

import { clientTypes } from "../constants";

type Props = {
  value: ClientType;
  onChange: (value: ClientType) => void;
};

export function ClientTypeSelector({ value, onChange }: Props) {
  return (
    <fieldset>
      <legend className="mb-4 text-sm text-white/65">
        Você está entrando em contato como:
      </legend>

      <div className="flex flex-wrap gap-3">
        {clientTypes.map(({ value: option, label }) => {
          const isActive = value === option;

          return (
            <label
              key={option}
              className={`cursor-pointer border px-5 py-3 text-sm transition ${
                isActive
                  ? "border-white bg-white text-(--color-light-black)"
                  : "border-white/20 text-white/65 hover:border-white/60"
              }`}
            >
              <input
                type="radio"
                name="client_type"
                value={option}
                checked={isActive}
                onChange={() => onChange(option)}
                className="sr-only"
              />

              {label}
            </label>
          );
        })}
      </div>
    </fieldset>
  );
}
