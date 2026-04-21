import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const loadFonts = async () => {
      try {
        // Create a FontFace for each variation
        const regular = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Regular.ttf") format("truetype")',
          { weight: '400', style: 'normal' }
        );
        const bold = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Bold.ttf") format("truetype")',
          { weight: '700', style: 'normal' }
        );
        const italic = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Italic.ttf") format("truetype")',
          { weight: '400', style: 'italic' }
        );
        const boldItalic = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-BoldItalic.ttf") format("truetype")',
          { weight: '700', style: 'italic' }
        );

        // Load all fonts
        const loadedFonts = await Promise.all([
          regular.load(),
          bold.load(),
          italic.load(),
          boldItalic.load(),
        ]);

        // Add them to the document
        loadedFonts.forEach(font => document.fonts.add(font));

        // Wait for fonts to be ready
        await document.fonts.ready;

        // Verify the font is available
        const isLoaded = document.fonts.check('12px "Zaroxi Mono"');
        if (isLoaded) {
          document.body.classList.add('fonts-loaded');
          setFontsLoaded(true);
        } else {
          setFontsLoaded(false);
        }
      } catch (error) {
        console.error('Failed to load fonts:', error);
        setFontsLoaded(false);
      }
    };

    loadFonts();
  }, []);

  // While fonts are loading, we could show a loading indicator
  // For now, just return null
  return null;
}
