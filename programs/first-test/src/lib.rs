use anchor_lang::prelude::*;
use anchor_spl::token::*; //importamos todo lo relacionado con el mint account
use anchor_spl::associated_token::*; //importamos todo lo relacionado con el token account

/*
- No es necesario crear una estructura de la cuenta especifica, ya que esta nos la facilita la libreria
*/

declare_id!("7umLstBrAY48SbsRCdqVMqLvREDCZtY67dxHdNfs87Kg");

#[program]
pub mod tokens {

    use super::*; // tiene acceso a lo que declaremos fuera del módulo

    //5. Crear funciones
    pub fn create_token_mint(_ctx: Context<CreateToken>) -> Result<()>{ //Al indicar _ctx, hacemos que ctx sea una variable inusada
        //anchor se ocupa de ?definir la logica?
        Ok(())
    }


}

//1. Crear contexto de la instrucción de crear un token
#[derive(Accounts)]
pub struct CreateToken<'info>{
    //2. Definir cuentas
    #[account(init, payer=authority, mint::decimals = 2, mint::authority = authority)]
    pub mint_account: Account<'info, Mint>, //Mint viene de anchor_spl::token, y nos trae toda la structura para una cuenta mint
    //6. Definimos cuenta para `token account`
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = authority)] //init_if_needed -> inicia la cuenta si todavia no se ha iniciado antes
    pub token_account: Account<'info, TokenAccount>,


    #[account(mut)]
    pub authority: Signer<'info>, //Cuenta para almacenar la autoridad

    //3. Definir programas asociados para ejecutar la instrucción
    pub system_program: Program<'info, System>, 
    pub token_program: Program<'info, Token>, // inicializa el `token mint`
    //7. Definir programa asociado a `token account`
    pub associated_token_program: Program<'info, AssociatedToken>,

    //4. Variables asociadas a la instrucción
    pub rent: Sysvar<'info, Rent>, // Para que anchor sepa cual es el valor actual de la renta, y pueda hacer los calculos vinculados a las fees

}

