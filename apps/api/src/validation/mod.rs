use std::borrow::Cow;

use rust_decimal::Decimal;
use uuid::Uuid;
use validator::ValidationError;

pub fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(validation_error(
            "blank",
            "Este campo não pode ficar vazio.",
        ));
    }
    Ok(())
}

pub fn validate_password(value: &str) -> Result<(), ValidationError> {
    let normalized = value.trim();

    if normalized.chars().count() < 8 {
        return Err(validation_error(
            "weak_password",
            "A senha deve ter pelo menos 8 caracteres.",
        ));
    }

    if normalized.chars().count() > 15 {
        return Err(validation_error(
            "weak_password",
            "A senha deve ter no máximo 15 caracteres.",
        ));
    }

    Ok(())
}

pub fn validate_not_nil_uuid(value: &Uuid) -> Result<(), ValidationError> {
    if value.is_nil() {
        return Err(validation_error("nil_uuid", "Identificador inválido."));
    }
    Ok(())
}

pub fn validate_positive_decimal(value: &Decimal) -> Result<(), ValidationError> {
    if value <= &Decimal::ZERO {
        return Err(validation_error(
            "non_positive",
            "O valor deve ser maior que zero.",
        ));
    }
    Ok(())
}

pub fn validate_percentage(value: &Decimal) -> Result<(), ValidationError> {
    if value < &Decimal::ZERO || value > &Decimal::ONE_HUNDRED {
        return Err(validation_error(
            "out_of_range",
            "O valor deve estar entre 0 e 100.",
        ));
    }
    Ok(())
}

pub fn validate_positive_i16(value: i16) -> Result<(), ValidationError> {
    if value <= 0 {
        return Err(validation_error(
            "non_positive",
            "O valor deve ser maior que zero.",
        ));
    }
    Ok(())
}

pub fn validate_non_negative_i16(value: i16) -> Result<(), ValidationError> {
    if value < 0 {
        return Err(validation_error(
            "negative",
            "O valor não pode ser negativo.",
        ));
    }
    Ok(())
}

pub fn validate_positive_i64(value: i64) -> Result<(), ValidationError> {
    if value <= 0 {
        return Err(validation_error(
            "non_positive",
            "O valor deve ser maior que zero.",
        ));
    }
    Ok(())
}

pub fn validate_cnpj(value: &str) -> Result<(), ValidationError> {
    if value.len() != 14 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(validation_error(
            "invalid_cnpj",
            "CNPJ deve conter 14 dígitos.",
        ));
    }

    let digits = value
        .bytes()
        .map(|byte| (byte - b'0') as u32)
        .collect::<Vec<_>>();
    if digits.iter().all(|digit| *digit == digits[0]) {
        return Err(validation_error("invalid_cnpj", "CNPJ inválido."));
    }

    let first_weights = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let second_weights = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let check_digit = |digits: &[u32], weights: &[u32]| {
        let remainder = digits
            .iter()
            .zip(weights)
            .map(|(digit, weight)| digit * weight)
            .sum::<u32>()
            % 11;
        if remainder < 2 { 0 } else { 11 - remainder }
    };

    if check_digit(&digits[..12], &first_weights) != digits[12]
        || check_digit(&digits[..13], &second_weights) != digits[13]
    {
        return Err(validation_error("invalid_cnpj", "CNPJ inválido."));
    }
    Ok(())
}

pub fn validate_cpf(value: &str) -> Result<(), ValidationError> {
    if value.len() != 11 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(validation_error(
            "invalid_cpf",
            "CPF deve conter 11 dígitos.",
        ));
    }

    let digits = value
        .bytes()
        .map(|byte| (byte - b'0') as u32)
        .collect::<Vec<_>>();
    if digits.iter().all(|digit| *digit == digits[0]) {
        return Err(validation_error("invalid_cpf", "CPF inválido."));
    }

    let first_sum = digits[..9]
        .iter()
        .enumerate()
        .map(|(index, digit)| digit * (10 - index as u32))
        .sum::<u32>();
    let second_sum = digits[..10]
        .iter()
        .enumerate()
        .map(|(index, digit)| digit * (11 - index as u32))
        .sum::<u32>();
    let first_digit = (first_sum * 10) % 11 % 10;
    let second_digit = (second_sum * 10) % 11 % 10;

    if first_digit != digits[9] || second_digit != digits[10] {
        return Err(validation_error("invalid_cpf", "CPF inválido."));
    }
    Ok(())
}

pub fn validate_brazilian_phone(value: &str) -> Result<(), ValidationError> {
    if !(10..=11).contains(&value.len()) || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(validation_error(
            "invalid_phone",
            "Telefone deve conter de 10 a 11 dígitos.",
        ));
    }
    Ok(())
}

pub fn validate_state_code(value: &str) -> Result<(), ValidationError> {
    if value.len() != 2 || !value.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return Err(validation_error(
            "invalid_state",
            "Estado deve conter duas letras maiúsculas.",
        ));
    }
    Ok(())
}

fn validation_error(code: &'static str, message: &'static str) -> ValidationError {
    let mut error = ValidationError::new(code);
    error.message = Some(Cow::Borrowed(message));
    error
}

#[cfg(test)]
mod tests {
    use super::{validate_cnpj, validate_cpf, validate_password};

    #[test]
    fn cpf_and_cnpj_checksums_are_verified() {
        assert!(validate_cpf("52998224725").is_ok());
        assert!(validate_cpf("52998224724").is_err());
        assert!(validate_cnpj("11222333000181").is_ok());
        assert!(validate_cnpj("11222333000182").is_err());
    }

    #[test]
    fn password_length_ignores_surrounding_whitespace() {
        assert!(validate_password("  12345678  ").is_ok());
        assert!(validate_password("       1234567 ").is_err());
    }
}
