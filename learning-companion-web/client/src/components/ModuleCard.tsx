import React from 'react';
import type { LearningModule, ModuleTasks } from '../types';
import { getLanguageTheme, type LanguageTheme } from '../themes';

interface ModuleCardProps {
  module: LearningModule;
  theme?: LanguageTheme;
  onUpdateProgress: (moduleId: string, taskType: string) => void;
  onStartPractice: (module: LearningModule) => void;
  onViewContent: (moduleId: string, contentType: string) => void;
}

export const ModuleCard: React.FC<ModuleCardProps> = ({
  module,
  theme,
  onUpdateProgress,
  onStartPractice,
  onViewContent
}) => {
  // Get theme, fallback to module language
  const cardTheme = theme || getLanguageTheme(module.language || 'rust');

  const taskLabels: (keyof ModuleTasks)[] = ['concept', 'examples', 'exercises', 'project', 'checklist'];
  const taskNames = {
    concept: '📖 概念学习',
    examples: '💻 代码示例',
    exercises: '✏️ 练习题',
    project: '📦 综合练习',
    checklist: '✅ 自检通过',
  };

  const getContentType = (task: keyof ModuleTasks): string => {
    switch (task) {
      case 'concept':
        return 'readme';
      case 'exercises':
        return 'exercises';
      case 'project':
        return 'project';
      case 'examples':
      case 'checklist':
      default:
        return task as string;
    }
  };

  const handleTaskClick = (task: keyof ModuleTasks) => {
    const contentType = getContentType(task);
    if (contentType === 'examples') {
      onViewContent(module.id, 'examples');
    } else {
      onViewContent(module.id, contentType);
    }

    if (!module.tasks[task]) {
      onUpdateProgress(module.id, task);
    }
  };

  return (
    <div
      className="mb-6 hover:shadow-xl transition-shadow duration-300 rounded-lg p-6"
      style={{
        borderLeft: `4px solid ${cardTheme.primary}`,
        backgroundColor: cardTheme.bg,
        borderTop: `1px solid ${cardTheme.primary}33`,
        borderRight: `1px solid ${cardTheme.primary}33`,
        borderBottom: `1px solid ${cardTheme.primary}33`
      }}
    >
      <div className="flex justify-between items-start mb-4">
        <div>
          <div className="flex items-center gap-2 mb-1">
            <span className="text-2xl">{cardTheme.emoji}</span>
            <h3 className="text-2xl font-bold" style={{ color: cardTheme.primary }}>
              {module.name}
            </h3>
          </div>
          <p className="text-sm" style={{ color: cardTheme.text, opacity: 0.7 }}>
            进度: {module.progress.toFixed(1)}%
          </p>
        </div>
        <div style={{ color: cardTheme.primary }}>
          <svg className="w-12 h-12" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2L2 7l10 5 10-5-10 5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
        </div>
      </div>

      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2" style={{ color: cardTheme.text }}>
          <span>掌握程度</span>
          <span className="font-semibold">{module.progress.toFixed(0)}%</span>
        </div>
        <div className="w-full rounded-full h-3" style={{ backgroundColor: `${cardTheme.primary}33` }}>
          <div
            className="h-3 rounded-full transition-all duration-500"
            style={{
              width: `${module.progress}%`,
              backgroundColor: cardTheme.primary
            }}
          />
        </div>
      </div>

      <div className="space-y-3">
        <h4 className="font-semibold mb-3" style={{ color: cardTheme.text }}>学习任务</h4>
        {taskLabels.map((task) => (
          <button
            key={task}
            onClick={() => handleTaskClick(task)}
            className="w-full flex items-center justify-between p-3 rounded-lg hover:opacity-80 transition-opacity text-left"
            style={{
              backgroundColor: `${cardTheme.primary}15`
            }}
          >
            <div className="flex items-center gap-3">
              <span className={module.tasks[task] ? 'text-green-600 text-xl' : 'text-gray-400 text-xl'}>
                {module.tasks[task] ? '✅' : '⭕'}
              </span>
              <span
                className={module.tasks[task] ? 'line-through' : ''}
                style={module.tasks[task] ? { color: cardTheme.text, opacity: 0.5 } : { color: cardTheme.text }}
              >
                {taskNames[task]}
              </span>
            </div>
            {module.tasks[task] && (
              <span className="text-green-600 text-sm font-medium">已完成</span>
            )}
          </button>
        ))}
      </div>

      <div className="flex gap-3 mt-6">
        <button
          onClick={() => onStartPractice(module)}
          className="flex-1 py-2 px-4 rounded-lg font-semibold text-white transition-opacity hover:opacity-90"
          style={{ backgroundColor: cardTheme.primary }}
        >
          📝 开始练习
        </button>
      </div>
    </div>
  );
};
