# Manejador de eventos
## Introducción
Ejercicio final del curso de Heavy Duty Camp, Primeros pasos en Solana con Rust.

Consiste en elaborar un proyecto de tu gusto para finalizar el bootcamp. Yo he elejido un manejador de loterias descentralizado.


### Resumen
#### Descripción general
Manejador de loterias descentralizado. Permite:

- Crear loterias - 🏗️ solo al dueño del programa
- Vender boletos (tokens no fungibles🏗️)

- Retirar fondos 🏗️❓
- Repartir los premios secundarios de la loteria 🏗️❓⚠️
- Destruir tokens de loteria 🏗️❓

- Participar como colaboradores ⚠️
- Distribuir ganancias del evento ⚠️

Los participadores obtendran tokens, de cierta loteria escpecifica, por ejemplo LOTERIA A. 
Una vez vendidos todos los boletos, se finalizara el evento y el usuario premiado obtendrá el premio.

Los tokens de cierta loteria, tendran un precio 1:x de la moneda que se vaya aceptar. Por ejemplo, si se acepta USDC para comprar las 'colaboraciones', tu token siempre tendra el valor de x USDC; si se hace con Solana, siempre tendra el valor x de Solana. Tambien tendrán un numero máximo (99e).

Cuando se termine el sorteo, se repartiran los premios 'boveda de la loteria'.

Para ello:
- Existira una 'boveda de la loteria', donde se guardaran los ingresos de la venta de 'tokens de la loteria'.🏗️
- Existira una 'boveda de ganancias', donde se guardaran los sobrantes de cada loteria. ⚠️
    - Vamos a suponer que esta cuenta PDA, nunca se autodestruira, osea siempre existira una vez creada y sera comun de cada token ⚠️
    - No va existir 'boveda de ganancias' ya que una vez se retiren los fondos, se eliminaran las bovedas 


Las ganancias se dividiran entre los colaboradores. ⚠️

##### Leyenda
- ⚠️ Desarrollar en futuras versiones
- 🏗️ Diferente al ejemplo pero 'aceptado' para intentarlo en esta version
- ❌ Diferente al ejemplo y eliminado
- ❓ Dudas de si introducir
#### Tipos de loteria
##### Cantidad boletos
- Rapida: 9 boletos
- Normal: 99 boletos
- Grande: 999 boletos
- Especiales: 9^ boletos ⚠️
##### Precio boleto
El precio el boleto siempre tendra un precio fijo, marcado al iniciar la loteria
##### Premios
Se escojera un numero aleatoriamente entre todos los boletos. El reintegro se refiere al ultimo numero del numero premiado (ej->89, reintegro: 9), del cual siempre abran 8 + el numero premiado.
- **Rapida:** 1 premio principal. 0'7^ de lo recaudado -> *10USDT * 9 = 90 * 0.7^ = 70USDT, lo mismo para cualquier moneda*
- **Normal y grande:** 1 premio principal + 9 premios secundarios(reintegro)
    - Ganador principal: 0'60^ de lo recaudado + 1 premio secundario-> *10USDT * 99 = 990 * 0.60^ = 600USDT, lo mismo para cualquier moneda*
    - Premio secundario: ~ valor 2x lo apostado -> *1SOL * 2 * <precio_actual>200usdc = valor:400usdc*
        - Si el valor supera los $40, se entregara el valor en una de estas opciones, por orden de posibilidad -> 
            1. Se enviara un regalo fisico valorado aproximadamente en el valor del premio secundario. 
            2. Se entragara dicho valor en distintas monedas del ecosistema.
            3. Se entragara dicho valor en la moneda apostada.
        - Si el valor no supera los $40, se entregara el valor en una de estas opciones, por orden de posibilidad ->
            1. Se entragara dicho valor en distintas monedas del ecosistema.
            2. Se entragara dicho valor en la moneda apostada.
        

### Instrucciones
- Crear una loteria
- Eliminar loteria
- Finalizar un evento ❓ Parar loteria -> Crearemos esta funcion como medida de seguridad,🧠 en el futuro se puede crear una instruccion que permita retirar fondos del evento si el evento esta parado
🏗️⬇️
- Comprar tokens del evento (sponsor) ⚠️
- Comprar boletos
- Retirar fondos del evento ❌/⚠️
- Retirar ganancias del evento 

#### Funciones diferentes a las instrucciones (auto ejecutables❓) 🏗️
Se ejecutara automaticamente al vender-se los boletos.
- Finalizar un evento
- Traspasar premio principal
- Traspasar fondos a 'boveda de ganancias'


#### Crear una lotería
Instrucción para crear las cuentas necesarias, asi como almacenar toda la informacion necesaria.
##### Información
- **Loteria**: Nombre (40), id(16), descripcion(150), precio token(u64), activo(bool), token_aceptado(pubkey), autoridad(pubkey), bump-s('varios'), tipo_loteria(TipoLoteria)
- **TipoLoteria**: ENUM -> Fast(9), Normal(99), Big(999)
##### Cuentas PDA
- Token del evento
- Boveda de la loteria
- Boveda de ganancias ⚠️
#### Finalizar lotería
Instrucción para eliminar una loteria. Para realizar esta accion, la 'boveda de la loteria', debera no tener fondos. Al eliminar esta cuenta, se debe hacer burn 🏗️, de los tokens creados.
Tambien se 'cancela' el token del evento
##### Información

##### Cuentas PDA



#### Parar lotería
Instrucción para parar la lotería. Esta instruccion simplemente inabilitara o habilitara otras instrucciones. *Switcher*
##### Información
##### Cuentas PDA

### Dudas
- [ ] Al cerrar una cuenta PDA con data asignada, tipo la cuenta `evento`, se elimina la data de la blockchain
- [ ] ❌😅 Al cerrar una token_account PDA, como la `boveda_evento`, esta se 'volvera' a utilizar al asignarle las mismas semillas. Es posible mantener los tokens. 'Entiendo que no' -> Se podria utilizar una cuenta 'común' del programa ('boveda ganancias USDC', 'boveda ganancias SOL', etc..) para cada token.

### To Do
