# 🧪 Plan de Pruebas Manuales — RPlayer v0.5.0

Este documento contiene una guía detallada de verificación paso a paso para probar manualmente el **100% de las funcionalidades** del reproductor **RPlayer** en entornos Linux (Fedora/Ubuntu/Debian), Windows y macOS.

---

## 📋 Índice de Pruebas

1. [🎬 Módulo 1: Carga y Control de Reproducción](#-módulo-1-carga-y-control-de-reproducción)
2. [🎛️ Módulo 2: Centro de Audio y Sonido](#️-módulo-2-centro-de-audio-y-sonido)
3. [🎬 Módulo 3: Centro de Video y Subtítulos](#-módulo-3-centro-de-video-y-subtítulos)
4. [⚙️ Módulo 4: Centro de Herramientas y Ajustes](#️-módulo-4-centro-de-herramientas-y-ajustes)
5. [📋 Módulo 5: Lista de Reproducción (Playlist)](#-módulo-5-lista-de-reproducción-playlist)
6. [🌗 Módulo 6: Sistema de Temas y Legibilidad Visual](#-módulo-6-sistema-de-temas-y-legibilidad-visual)

---

## 🎬 Módulo 1: Carga y Control de Reproducción

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-01** | **Apertura de Archivo** | 1. Hacer clic en `📂 Abrir` en la barra superior.<br>2. Seleccionar un archivo multimedia (`.mkv`, `.mp4`, `.mp3`, `.flac`). | El archivo se carga de inmediato, el título aparece en el encabezado y el video comienza a reproducirse sin pantalla negra. |
| **TC-02** | **Reproducir / Pausar** | 1. Clic en `⏸ Pausar` o pulsar la tecla `Espacio`.<br>2. Clic en `▶ Reproducir`. | El video se congela inmediatamente en el fotograma actual y se reanuda al volver a presionar. |
| **TC-03** | **Búsqueda por Barra de Progreso** | 1. Hacer clic en cualquier punto de la barra de progreso inferior.<br>2. Arrastrar el deslizador de izquierda a derecha. | El video salta de forma instantánea (*keyframe seek*) a la posición indicada sin que la pantalla se apague ni se vuelva blanca. |
| **TC-04** | **Saltos Rápido ±10s** | 1. Hacer clic en `⏪ -10s`.<br>2. Hacer clic en `⏩ +10s`. | El contador de tiempo retrocede o avanza exactamente 10 segundos por cada clic de forma fluida. |
| **TC-05** | **Control de Volumen y Mute** | 1. Arrastrar el slider de volumen.<br>2. Clic en el botón `🔊` / `🔇`. | El nivel de audio cambia progresivamente. Al silenciar, cambia a `🔇` y enmudece la salida de audio. |
| **TC-06** | **Velocidad de Reproducción** | 1. Desplegar el selector de velocidad en la barra inferior.<br>2. Seleccionar `0.5x`, `1.5x` o `2.0x`. | El audio y el video aceleran o desaceleran al ritmo exacto seleccionado sin pérdida de tono. |
| **TC-07** | **Bucle de Repetición A-B** | 1. Presionar `🔂 A-B` en la posición inicial (Punto A).<br>2. Presionar nuevamente más adelante (Punto B). | El reproductor cicla automáticamente entre el Punto A y el Punto B sin detenerse. Clic adicional limpia el bucle. |

---

## 🎛️ Módulo 2: Centro de Audio y Sonido

> **Acceso**: Clic en el botón `🎛️ Audio` en la barra superior.

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-08** | **Ecualizador PEQ (Filtros)** | 1. Marcar el checkbox *"Activar ecualizador paramétrico"*.<br>2. Mover los sliders de frecuencia (`60Hz` a `16kHz`). | Se percibe el cambio acústico en los tonos graves, medios y agudos en tiempo real. |
| **TC-09** | **Presets de Ecualizador** | 1. Desplegar la lista de presets.<br>2. Seleccionar `Bass Boost`, `Rock` o `Vocal`. | El desplegable es 100% legible (texto blanco sobre fondo oscuro en modo oscuro) y los sliders adoptan los valores del preset. |
| **TC-10** | **Pistas de Audio Alternativas** | 1. Ir a la pestaña `🎧 Pistas de Audio`.<br>2. Seleccionar una pista secundaria en archivos multilenguaje. | La pista de audio conmuta inmediatamente al idioma o pista seleccionada. |
| **TC-11** | **Desfase de Audio** | 1. Mover el slider *"Desfase de Audio"* entre `-5.0s` y `+5.0s`. | El sonido se retarda o adelanta respecto a los labios de los personajes en la pantalla. |
| **TC-12** | **Supresión Vocal (Karaoke)** | 1. Ir a la pestaña `🎤 Karaoke & Tono`.<br>2. Activar *"Supresión Vocal"*. | Las frecuencias focales del centro de la canción (voces) se atenuan significativamente. |
| **TC-13** | **Ajuste de Tono (Pitch Shift)** | 1. Mover el slider de *"Pitch Shift"* de `-6.0` a `+6.0` semitonos. | El tono musical sube o baja de escala manteniendo la misma velocidad de reproducción. |

---

## 🎬 Módulo 3: Centro de Video y Subtítulos

> **Acceso**: Clic en el botón `🎬 Video` en la barra superior.

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-14** | **Subtítulos Integrados** | 1. Abrir un video con subtítulos integrados.<br>2. Ir a la pestaña `💬 Subtítulos` y seleccionar una pista. | Los subtítulos aparecen en la parte inferior de la imagen de video. |
| **TC-15** | **Subtítulos Externos** | 1. Clic en `📁 Cargar Archivo (.srt / .vtt)`.<br>2. Elegir un archivo de subtítulo local. | El archivo `.srt` o `.vtt` se vincula e interpreta sobre la reproducción activa. |
| **TC-16** | **Desfase de Subtítulos** | 1. Mover el slider *"Desfase de Subtítulos"* entre `-5.0s` y `+5.0s`. | El texto de los subtítulos aparece antes o después del diálogo escuchado. |
| **TC-17** | **Buscador OpenSubtitles** | 1. Escribir el título de una película en el buscador de OpenSubtitles.<br>2. Presionar `🔍 Buscar`. | Se consulta la API en segundo plano sin congelar la interfaz de usuario. |
| **TC-18** | **Ajustes de Imagen (Brillo/Contraste)** | 1. Ir a la pestaña `🎨 Ajustes de Imagen`.<br>2. Mover sliders de *Brillo*, *Contraste*, *Saturación*, *Tono* y *Gamma*. | La imagen del video responde en tiempo real a los cambios de colorimetría. |
| **TC-19** | **Restablecer Imagen** | 1. Presionar `🔄 Restablecer Valores por Defecto`. | Todos los sliders de imagen vuelven a `0` y el video recupera sus colores originales. |

---

## ⚙️ Módulo 4: Centro de Herramientas y Ajustes

> **Acceso**: Clic en el botón `⚙️ Herramientas` en la barra superior.

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-20** | **Añadir Marcador** | 1. Ir a la pestaña `🔖 Marcadores & Notas`.<br>2. Clic en `➕ Añadir Marcador Actual`. | Se registra la marca de tiempo exacta con su etiqueta editable. |
| **TC-21** | **Saltar a Marcador** | 1. Hacer clic en `▶ Saltar` en cualquier marcador guardado. | La reproducción salta inmediatamente a la marca de tiempo del archivo. |
| **TC-22** | **Eliminar Marcador** | 1. Hacer clic en el icono `🗑️` en una fila de marcador. | La entrada se elimina de la lista y de la persistencia JSON. |
| **TC-23** | **Recorte de Video (Trim)** | 1. Ir a la pestaña `✂️ Recorte sin Pérdida`.<br>2. Fijar inicio y fin y presionar `✂️ Exportar Clip Recortado`. | Se invoca `ffmpeg` para extraer el segmento sin perder calidad ni re-codificar. |
| **TC-24** | **Ficha Técnica (MediaInfo)** | 1. Ir a la pestaña `ℹ️ Ficha Técnica`. | Se muestran la resolución exactas, códecs de video/audio, bitrate, contenedor y FPS del archivo. |
| **TC-25** | **Servidor de Control Remoto HTTP** | 1. Ir a la pestaña `⚙️ Red & Preferencias`.<br>2. Clic en `▶ Iniciar Servidor Remoto (Puerto 8080)`.<br>3. Abrir `http://localhost:8080` en un navegador. | Aparece el panel web remoto para pausar, reproducir y controlar el volumen desde el teléfono/navegador. |
| **TC-26** | **Temporizador de Apagado** | 1. Seleccionar un temporizador (`15 min`, `30 min`, `45 min`, `60 min`). | La cuenta regresiva se activa y detiene la reproducción al finalizar el tiempo. |

---

## 📋 Módulo 5: Lista de Reproducción (Playlist)

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-27** | **Alternar Panel Lateral** | 1. Clic en `📋 Playlist` en la barra superior. | El panel lateral derecho se despliega de forma animada sin alterar la relación de aspecto del video. |
| **TC-28** | **Selección de Pista** | 1. Clic en cualquier archivo de la lista de reproducción. | El archivo seleccionado se carga y comienza a reproducirse de inmediato. |

---

## 🌗 Módulo 6: Sistema de Temas y Legibilidad Visual

| ID | Funcionalidad | Pasos de Verificación | Resultado Esperado |
| :-: | :--- | :--- | :--- |
| **TC-29** | **Alternancia de Modo Oscuro / Claro** | 1. Clic en `☀️ Claro` / `🌙 Oscuro` en la esquina superior derecha. | Toda la interfaz conmuta instantáneamente entre el tema oscuro carbón y el tema claro brillante. |
| **TC-30** | **Legibilidad de Desplegables (`<select>`)** | 1. Abrir cualquier lista desplegable (ej: presets del ecualizador o velocidad) en Modo Oscuro.<br>2. Pasar el cursor sobre las opciones. | **Caja y Menú**: El cuadro del selector tiene fondo oscuro `#1a1d24`, texto blanco `#f3f4f6` y una flecha SVG vectorial. Las opciones emergentes se leen con contraste perfecto. |
| **TC-31** | **Cierre Limpio de Modales (1 Clic)** | 1. Abrir cualquier modal (`Audio`, `Video`, `Herramientas`).<br>2. Clic en `✕` o sobre la capa de fondo (*backdrop*). | El modal se cierra al instante en **1 solo clic** sin capas de fondo duplicadas ni quedarse "pegado". |
