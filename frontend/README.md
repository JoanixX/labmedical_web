# Frontend LabMedical

Frontend de la plataforma de catalogo medico B2B, construido con Astro y TypeScript.

## Stack Tecnologico

- **Framework**: Astro 4.x
- **Lenguaje**: TypeScript
- **Estilos**: CSS vanilla
- **Hosting**: Vercel

## Estructura del Proyecto

```
frontend/
├── src/
│   ├── components/     # componentes reutilizables
│   ├── layouts/        # layouts de pagina
│   ├── pages/          # paginas de la aplicacion
│   ├── lib/            # utilidades y cliente api
│   └── env.d.ts        # definiciones de tipos para variables de entorno
├── public/             # archivos estaticos
├── astro.config.mjs    # configuracion de astro
├── tsconfig.json       # configuracion de typescript
├── package.json        # dependencias
├── .env.example        # plantilla de variables de entorno
└── README.md
```

## Instalacion

1. Instalar dependencias:

```bash
npm install
```

2. Configurar variables de entorno:

```bash
cp .env.example .env
```

3. Editar `.env` con la URL del backend:

```env
PUBLIC_API_URL=http://localhost:3000
```

## Desarrollo

Iniciar servidor de desarrollo:

```bash
npm run dev
```

El sitio estara disponible en `http://localhost:4321`

## Build

Generar build de produccion:

```bash
npm run build
```

Previsualizar build:

```bash
npm run preview
```

## Variables de Entorno

- `PUBLIC_API_URL`: URL del backend API (requerida)

## Paginas Disponibles

- `/` - Pagina de inicio
- `/productos` - Listado de productos
- `/productos/[slug]` - Detalle de producto
- `/cotizacion` - Formulario de cotizacion
- `/admin` - Panel administrativo (proximamente)

## Deployment

El frontend esta configurado para deployment en Vercel:

1. Conectar repositorio en Vercel
2. Configurar variable de entorno `PUBLIC_API_URL` con la URL del backend en produccion
3. Deploy automatico en cada push a main

## Licencia

Propietario - LabMedical
