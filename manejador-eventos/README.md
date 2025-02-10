# Manejador de eventos
## Introducción
### Resumen
Manejador de eventos descentralizado. Permite:
- Crear eventos
- Participar como colaboradores
- Vender entradas
- Distribuir ganancias del evento
Los colaboradores obtendran tokens, de cierto evento escpecifico, por ejemplo EVENTO A. Una vez finalizado el evento se podrá obtener parte de los beneficios de dicho evento.
Es como un crowfunding.

Los tokens de cierto evento, tendran un precio 1:1 de la moneda que se vaya aceptar. Por ejemplo, si se acepta USDC para comprar las 'colaboraciones', tu token siempre tendra el valor de un USDC; si se hace con Solana, siempre tendra el valor de Solana.
Para ello:
- Existira una 'boveda del evento', donde se guardaran los ingresos de la venta de 'tokens de colaboración'.
- Existira una 'boveda de ganancias', donde se guardaran los ingresos de la venta de 'entradas al evento'.

Las ganancias se dividiran entre los colaboradores.
### Instrucciones
- Crear un evento
- Eliminar un evento
- Comprar tokens del evento (sponsor)
- Comprar entradas
- Retirar fondos del evento
- Finalizar un evento
- Retirar ganancias del evento
#### Crear un evento
##### Información
Nombre, identificador, descripcion, precio token evento, precio entradas
##### Cuentas PDA
- Token del evento
- Boveda del evento
- Boveda de ganancias

