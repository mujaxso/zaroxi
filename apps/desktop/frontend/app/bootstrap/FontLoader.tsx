import { useEffect, useState } from 'react';

/**
 * FontLoader
 *
 * This component is responsible for:
 * 1. Importing the CSS file that contains the @font‑face declarations
 *    (the CSS is tree‑shaken by Vite, so we need a dynamic import).
 * 2. Waiting for the fonts to be ready via the CSS Font Loading API.
 * 3. Setting a `data-fonts-loaded` attribute on the document root so
 *    that CSS can apply a “fonts‑loaded” class to prevent FOUT.
 *
 * The actual @font‑face rules live in `lib/theme/fonts.css` and are
 * resolved via the Vite dev‑server root (`/fonts/…`) in both web
 * and desktop modes because Tauri serves the frontend dist at the
 * root path.
 */
export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    let cancelled = false;

    const loadFonts = async () => {
      try {
        // 1. Dynamically import the CSS so Vite includes it in the bundle
        await import('@/lib/theme/fonts.css');

        // 2. Wait for the browser to finish loading the font files
        await document.fonts.ready;

        // 3. Verify that our intended font family is actually available
        const fontFamily = 'JetBrainsMono Nerd Font';
        const isLoaded = document.fonts.check(`1em "${fontFamily}"`);

        if (!cancelled) {
          if (isLoaded) {
            document.documentElement.setAttribute('data-fonts-loaded', 'true');
            document.body.classList.add('fonts-loaded');
            setFontsLoaded(true);
          } else {
            // The font didn't load – log a warning but don't block the UI
            console.warn(
              `[FontLoader] Font "${fontFamily}" is not available. ` +
              'Falling back to the system monospace stack defined in CSS variables.'
            );
            // Still mark as ready so the UI renders
            setFontsLoaded(true);
          }
        }
      } catch (error) {
        if (!cancelled) {
          console.error('[FontLoader] Failed to load font CSS:', error);
          setFontsLoaded(true); // Don't block the UI
        }
      }
    };

    loadFonts();

    return () => {
      cancelled = true;
    };
  }, []);

  // While fonts are loading, we could show a loading indicator.
  // For now, just return null.
  return null;
}
