use anchor_lang::prelude::*;

/*
Es importante el nombre de este archivo
*/

#[error_code]
pub enum CodigoError {
    #[msg("Solo la autoridad de la lotería puede ejecutar esta instrucción.")]
    UsuarioNoAutorizado,

    #[msg("No puedes eliminar la lotería si la bóveda de la lotería no está vacía")]
    BovedaDelaLoteriaNoVacia,

    #[msg("No puedes eliminar la lotería si todavía quedan boletos por vender o hay usuarios con boletos comprados")]
    TokensDispobiles,
}
