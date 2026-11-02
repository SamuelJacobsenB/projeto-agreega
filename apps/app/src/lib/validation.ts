export function isValidCpf(value: string) {
  if (!/^\d{11}$/.test(value) || /^(\d)\1{10}$/.test(value)) {
    return false;
  }

  const digits = [...value].map(Number);
  const firstSum = digits
    .slice(0, 9)
    .reduce((sum, digit, index) => sum + digit * (10 - index), 0);
  const firstDigit = ((firstSum * 10) % 11) % 10;
  const secondSum = digits
    .slice(0, 10)
    .reduce((sum, digit, index) => sum + digit * (11 - index), 0);
  const secondDigit = ((secondSum * 10) % 11) % 10;

  return firstDigit === digits[9] && secondDigit === digits[10];
}

export function isValidCnpj(value: string) {
  if (!/^\d{14}$/.test(value) || /^(\d)\1{13}$/.test(value)) {
    return false;
  }

  const digits = [...value].map(Number);
  const calculateDigit = (values: number[], weights: number[]) => {
    const remainder =
      values.reduce((sum, digit, index) => sum + digit * weights[index], 0) %
      11;
    return remainder < 2 ? 0 : 11 - remainder;
  };
  const firstDigit = calculateDigit(
    digits.slice(0, 12),
    [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2],
  );
  const secondDigit = calculateDigit(
    digits.slice(0, 13),
    [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2],
  );

  return firstDigit === digits[12] && secondDigit === digits[13];
}
