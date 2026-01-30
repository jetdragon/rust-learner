export interface LanguageTheme {
  primary: string;
  secondary: string;
  accent: string;
  bg: string;
  text: string;
  emoji: string;
  name: string;
}

export const LANGUAGE_THEMES: Record<string, LanguageTheme> = {
  rust: {
    primary: '#dc5028',
    secondary: '#f97316',
    accent: '#ea580c',
    bg: '#fff7ed',
    text: '#7c2d12',
    emoji: '🦀',
    name: 'Rust'
  },
  python: {
    primary: '#3776ab',
    secondary: '#3b82f6',
    accent: '#2563eb',
    bg: '#eff6ff',
    text: '#1e3a8a',
    emoji: '🐍',
    name: 'Python'
  },
  go: {
    primary: '#00add8',
    secondary: '#06b6d4',
    accent: '#0891b2',
    bg: '#ecfeff',
    text: '#164e63',
    emoji: '🐹',
    name: 'Go'
  }
};

export function getLanguageTheme(language: string): LanguageTheme {
  return LANGUAGE_THEMES[language] || {
    primary: '#6b7280',
    secondary: '#9ca3af',
    accent: '#4b5563',
    bg: '#f3f4f6',
    text: '#1f2937',
    emoji: '📚',
    name: language.charAt(0).toUpperCase() + language.slice(1)
  };
}
