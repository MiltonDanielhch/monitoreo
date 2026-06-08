import adapter from '@sveltejs/adapter-static'; // <- Cambiado aquí

/** @type {import('@sveltejs/kit').Config} */
const config = {
    compilerOptions: {
        // Force runes mode for the project, except for libraries. Can be removed in svelte 6.
        runes: ({ filename }) => (filename.split(/[/\\]/).includes('node_modules') ? undefined : true)
    },
    kit: {
        // Configuración para el adaptador estático de producción
        adapter: adapter({
            pages: 'build',       // Directorio donde se guardarán los HTML/JS/CSS
            assets: 'build',
            fallback: 'index.html', // Importante: Habilita el modo Single Page App (SPA)
            precompress: false,
            strict: true
        })
    }
};

export default config;