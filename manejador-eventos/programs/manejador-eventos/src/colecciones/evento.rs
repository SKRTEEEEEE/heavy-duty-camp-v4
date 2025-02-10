use anchor_lang::prelude::*;
//Estructura
#[account]
#[derive(InitSpace)]
pub struct Evento{
    #[max_len(16)]
    pub id: String,

    #[max_len(40)]
    pub nombre: String,

    #[max_len(150)]
    pub descripcion: String,
    // precios, expresados en la unidad minima
    pub precio_entrada: u64,
    pub precio_token: u64,

    // estado del evento
    pub activo: bool,
    pub total_sponsors: u64,
    pub sponsors_actuales: u64,
    pub tokens_vendidos:u64,
    pub entradas_vendidas:u64,

    // creador evento
    pub autoridad: Pubkey,
    // token aceptado para el evento
    pub token_aceptado: Pubkey,
    
    // bumps cuentas pda
    pub bump_evento: u8, 
    pub bump_token_evento: u8,
    pub bump_boveda_evento: u8,
    pub bump_boveda_ganancias: u8,
}
// semillas extra para las PDAs
impl Evento {
    pub const SEMILLA_EVENTO: &'static str = "evento";
    pub const SEMILLA_TOKEN_EVENTO: &'static str = "token_evento";
    pub const SEMILLA_BOVEDA_EVENTO: &'static str = "boveda_evento";
    pub const SEMILLA_BOVEDA_GANANCIAS: &'static str = "boveda_ganancias";
}