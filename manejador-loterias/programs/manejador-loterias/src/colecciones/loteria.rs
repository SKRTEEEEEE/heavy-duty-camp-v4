use anchor_lang::prelude::*;

// Enum para los tipos de lotería
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub enum TipoLoteria {
    Fast(u64),
    Normal(u64),
    Big(u64),
}

impl TipoLoteria {
    // Función que mapea un valor u64 a una variante del enum
    pub fn from_u64(value: u64) -> Self {
        match value {
            9 => TipoLoteria::Fast(9),
            99 => TipoLoteria::Normal(99),
            999 => TipoLoteria::Big(999),
            _ => TipoLoteria::Fast(9), // Valor por defecto en caso de un valor no válido
        }
    }

    // Función para obtener el valor asociado a cada variante del enum
    pub fn valor(&self) -> u64 {
        match self {
            TipoLoteria::Fast(val) => *val,
            TipoLoteria::Normal(val) => *val,
            TipoLoteria::Big(val) => *val,
        }
    }
}

// Estructura de la lotería en Anchor
#[account]
#[derive(InitSpace)]
pub struct Loteria {
    #[max_len(16)]
    pub id: String,

    #[max_len(40)]
    pub nombre: String,

    #[max_len(150)]
    pub descripcion: String,

    // Precios expresados en la unidad mínima
    pub precio_token: u64,

    // Estado de la lotería
    pub activo: bool,

    // Cantidad de tickets vendidos
    pub tokens_vendidos: u64,

    // Tipo de lotería
    pub tipo_loteria: TipoLoteria,

    // Creador de la lotería
    pub autoridad: Pubkey,

    // Token aceptado para la lotería
    pub token_aceptado: Pubkey,

    // Bumps de las cuentas PDA
    pub bump_loteria: u8,
    pub bump_token_loteria: u8,
    pub bump_boveda_loteria: u8,
}

// Semillas para las PDAs
impl Loteria {
    pub const SEMILLA_LOTERIA: &'static str = "loteria";
    pub const SEMILLA_TOKEN_LOTERIA: &'static str = "token_loteria";
    pub const SEMILLA_BOVEDA_LOTERIA: &'static str = "boveda_loteria";
}
