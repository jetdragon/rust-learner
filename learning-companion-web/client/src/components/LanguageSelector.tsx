import React from 'react';
import { LANGUAGE_THEMES } from '../themes';

interface LanguageSelectorProps {
  onSelectLanguage: (language: string) => void;
}

export const LanguageSelector: React.FC<LanguageSelectorProps> = ({ onSelectLanguage }) => {
  const availableLanguages = [
    {
      id: 'rust',
      theme: LANGUAGE_THEMES.rust,
      description: '系统编程语言，安全高效',
      features: ['内存安全', '高性能', '并发编程']
    },
    {
      id: 'python',
      theme: LANGUAGE_THEMES.python,
      description: '简洁优雅，快速开发',
      features: ['易学易用', '生态丰富', '数据科学']
    },
    {
      id: 'go',
      theme: LANGUAGE_THEMES.go,
      description: '简洁高效，云原生',
      features: ['简洁语法', '高性能', '并发支持']
    }
  ];

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold text-warm-800 mb-4">
          🦀🐍🐹 多语言学习伴侣
        </h1>
        <p className="text-xl text-warm-600 mb-2">
          选择你想要学习的编程语言
        </p>
        <p className="text-sm text-warm-500">
          每种语言包含 12 个渐进式学习模块
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-6xl mx-auto">
        {availableLanguages.map((lang) => (
          <button
            key={lang.id}
            onClick={() => onSelectLanguage(lang.id)}
            className="text-left p-8 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-1"
            style={{
              borderLeft: `6px solid ${lang.theme.primary}`,
              backgroundColor: lang.theme.bg,
              borderTop: `2px solid ${lang.theme.primary}33`,
              borderRight: `2px solid ${lang.theme.primary}33`,
              borderBottom: `2px solid ${lang.theme.primary}33`
            }}
          >
            <div className="flex items-center gap-3 mb-4">
              <span className="text-5xl">{lang.theme.emoji}</span>
              <div>
                <h2 
                  className="text-3xl font-bold"
                  style={{ color: lang.theme.primary }}
                >
                  {lang.theme.name}
                </h2>
              </div>
            </div>

            <p 
              className="text-lg mb-4"
              style={{ color: lang.theme.text, opacity: 0.8 }}
            >
              {lang.description}
            </p>

            <div className="space-y-2 mb-6">
              {lang.features.map((feature, idx) => (
                <div 
                  key={idx}
                  className="flex items-center gap-2 text-sm"
                  style={{ color: lang.theme.text, opacity: 0.7 }}
                >
                  <span>✓</span>
                  <span>{feature}</span>
                </div>
              ))}
            </div>

            <div className="flex items-center justify-between pt-4 border-t-2">
              <span 
                className="text-sm font-medium"
                style={{ color: lang.theme.text }}
              >
                12 个学习模块
              </span>
              <span 
                className="px-4 py-2 rounded-lg text-white font-semibold"
                style={{ backgroundColor: lang.theme.primary }}
              >
                开始学习 →
              </span>
            </div>
          </button>
        ))}
      </div>

      <div className="text-center mt-12 text-warm-500 text-sm">
        <p>💡 提示：点击语言卡片即可开始学习该语言的模块</p>
      </div>
    </div>
  );
};
