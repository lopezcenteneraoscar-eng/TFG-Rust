Si lo hacemos a mano
Para generar el vcd hemos creado el sumador_tb

Y para compilarlo y que nos muestre el vcd hemos ejecutado los siguientes comandos

iverilog -o testbench sumadorRust.v sumador_tb.v
vvp testbench

Con esos nos generaba el fichero sumadorWave.vcd

Y luego solo lo visualizamos con gtkwave sumadoWave.vcd


Con el codigo implementado actualmente, el rust genera el vcd, y el testbench.

Para ejecutar el test usamos el comando

cargo run  -> y justo despues
cargo test

Hacemos esto porque el test suele necesitar usar el fichero .v que hemos generado para los test

Si ponemos println para depurar y queremos que se vean, usamos el comando

cargo test -- --nocapture

Y nos genera un fichero .vcd con los imputs de prueba que hemos metido nosotros.

Lo bueno es que lo hace tanto usando iverilog como rust, todo implementado en el codigo de rust
