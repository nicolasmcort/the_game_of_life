@echo off
title Simulación Autómata Celular de Conway
cls
if not exist the_game_of_life.exe (
    echo [ERROR] No se encuentra el ejecutable nativo "the_game_of_life.exe" en este directorio.
    echo Por favor, asegúrese de que el archivo .exe esté al lado de este lanzador.
    pause
    exit
)
the_game_of_life.exe
pause