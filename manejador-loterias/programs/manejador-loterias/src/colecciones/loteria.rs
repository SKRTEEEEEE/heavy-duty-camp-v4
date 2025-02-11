use anchor_lang::prelude::*;
//Estructura
#[account]
#[derive(InitSpace)]
pub struct Loteria {
    #[max_len(16)]
    pub id: String,

    #[max_len(40)]
    pub nombre: String,

    #[max_len(150)]
    pub descripcion: String,
    // precios, expresados en la unidad minima
    // pub precio_entrada: u64,
    pub precio_token: u64,

    // estado del loteria
    pub activo: bool,

    pub tokens_vendidos: u64,

    // creador loteria
    pub autoridad: Pubkey,
    // token aceptado para el loteria
    pub token_aceptado: Pubkey,

    // bumps cuentas pda
    pub bump_loteria: u8,
    pub bump_token_loteria: u8,
    pub bump_boveda_loteria: u8,
    // pub bump_boveda_ganancias: u8,
}
// semillas extra para las PDAs
impl Loteria {
    pub const SEMILLA_LOTERIA: &'static str = "loteria";
    pub const SEMILLA_TOKEN_LOTERIA: &'static str = "token_loteria";
    pub const SEMILLA_BOVEDA_LOTERIA: &'static str = "boveda_loteria";
}
