# Atajos de teclado — RPlayer

## Reproducción

| Tecla | Acción |
|-------|--------|
| `Espacio` | Play / Pausa |
| `N` | Siguiente pista |
| `P` | Pista anterior |
| `→` | Avanzar 5 segundos |
| `←` | Retroceder 5 segundos |
| `Shift+→` | Avanzar 60 segundos |
| `Shift+←` | Retroceder 60 segundos |
| `.` | Frame adelante (requiere pausa) |
| `,` | Frame atrás (requiere pausa) |
| `R` | Ciclar A-B loop: marcar A → marcar B → limpiar |

## Audio

| Tecla | Acción |
|-------|--------|
| `↑` | Subir volumen 5% |
| `↓` | Bajar volumen 5% |
| `M` | Silenciar / Dessilenciar |
| `+` | Aumentar velocidad 0.25× |
| `-` | Reducir velocidad 0.25× |
| `=` | Restablecer velocidad a 1× |

## Video y captura

| Tecla | Acción |
|-------|--------|
| `S` | Capturar frame PNG en ~/Pictures/RPlayer/ |
| `Ctrl+→` | Rotar video +90° |
| `Ctrl+←` | Rotar video -90° |

## Organización

| Tecla | Acción |
|-------|--------|
| `B` | Añadir marcador en la posición actual |

## Archivos

| Tecla | Acción |
|-------|--------|
| `Ctrl+O` | Abrir archivo |
| `Ctrl+U` | Abrir URL / Stream |

## Drag & Drop

Arrastra archivos de video, audio o .CDG directamente sobre la ventana para abrirlos o añadirlos a la playlist.

---

## Menús contextuales (clic derecho)

### Área de video

| Acción | Descripción |
|--------|-------------|
| Play / Pausa | Toggle de reproducción |
| Detener | Detiene y resetea posición |
| Saltar... | Submenú con +5s, +60s, -5s, -60s |
| Silenciar / Activar sonido | Toggle mute |
| Subir / Bajar volumen | ±5% |
| Normalización de volumen | Activa/desactiva loudnorm |
| Audio y subtítulos | Abre submenú con paneles de audio, subs, EQ, sync |
| Capturar frame | Guarda PNG |
| Picture-in-Picture | Ventana flotante always-on-top |
| Relación de aspecto | Auto / 4:3 / 16:9 / 21:9 / 1:1 |
| Imagen y video | Controles detallados (brillo/contraste/zoom/rotación) |
| Recortar video... | Abre panel de recorte |
| Convertir formato... | Abre panel de conversión |
| Información de medios | Muestra codec, resolución, bitrate, etc. |

### Ítem de playlist

| Acción | Descripción |
|--------|-------------|
| Reproducir | Reproduce este archivo |
| Mover arriba | Sube la posición en la lista |
| Mover abajo | Baja la posición en la lista |
| Añadir marcador | Crea marcador con la posición actual |
| Copiar ruta | Copia la ruta al portapapeles |
| Mostrar en explorador | Abre el directorio del archivo |
| Quitar de la lista | Elimina de la playlist (no del disco) |
| Limpiar lista | Vacía toda la playlist |

### Ítem del historial

| Acción | Descripción |
|--------|-------------|
| Abrir | Reproduce el archivo |
| Mostrar en explorador | Abre el directorio |
| Copiar ruta | Copia la ruta al portapapeles |
| Quitar del historial | Elimina solo esta entrada |
| Limpiar historial | Borra todo el historial |

### Marcador

| Acción | Descripción |
|--------|-------------|
| Ir a este marcador | Salta a la posición |
| Renombrar | Cambia la etiqueta |
| Eliminar marcador | Elimina el marcador |

---

## Notas de uso

- **A-B Loop**: Presiona `R` una vez para marcar el punto A, otra vez para marcar B, y una tercera vez para limpiar ambos. Los marcadores A y B aparecen como líneas en la seekbar.

- **Frame a frame**: Solo funciona con el video pausado. Útil para análisis de movimiento, sports, animación.

- **Captura de frames**: Se guarda en `~/Pictures/RPlayer/` (Linux) o `%USERPROFILE%\Pictures\RPlayer\` (Windows). El nombre incluye el timestamp.
