import React from 'react';
import type { LearningModule, ModuleTasks } from '../types';

interface ModuleCardProps {
  module: LearningModule;
  onUpdateProgress: (moduleId: string, taskType: string) => void;
  onStartPractice: (moduleId: string) => void;
}

export const ModuleCard: React.FC<ModuleCardProps> = ({ module, onUpdateProgress, onStartPractice }) => {
  const taskLabels: (keyof ModuleTasks)[] = ['concept', 'examples', 'exercises', 'project', 'checklist'];
  const taskNames = {
    concept: '概念学习',
    examples: '代码示例',
    exercises: '练习题',
    project: '综合练习',
    checklist: '自检通过',
  };

  return (
    <div className="card-warm mb-6 hover:shadow-xl transition-shadow duration-300">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-2xl font-bold text-warm-800 mb-1">{module.name}</h3>
          <p className="text-sm text-warm-600">进度: {module.progress.toFixed(1)}%</p>
        </div>
        <div className="text-warm-500">
          <svg className="w-12 h-12" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2L2 7l10 5 10-5-10 5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
        </div>
      </div>

      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2">
          <span>掌握程度</span>
          <span className="font-semibold">{module.progress.toFixed(0)}%</span>
        </div>
        <div className="w-full bg-warm-200 rounded-full h-3">
          <div
            className="progress-bar-warm h-3 rounded-full transition-all duration-500"
            style={{ width: `${module.progress}%` }}
          />
        </div>
      </div>

      <div className="space-y-3">
        <h4 className="font-semibold text-warm-700 mb-3">学习任务</h4>
        {taskLabels.map((task) => (
          <div key={task} className="flex items-center justify-between p-3 bg-warm-50 rounded-lg hover:bg-warm-100 transition-colors">
            <div className="flex items-center gap-3">
              <input
                type="checkbox"
                checked={module.tasks[task]}
                onChange={() => onUpdateProgress(module.id, task)}
                className="w-5 h-5 text-warm-600 rounded focus:ring-warm-500"
              />
              <span className={module.tasks[task] ? 'line-through text-warm-400' : 'text-warm-800'}>
                {taskNames[task]}
              </span>
            </div>
            {module.tasks[task] && <span className="text-green-600">✓</span>}
          </div>
        ))}
      </div>

      <div className="flex gap-3 mt-6">
        <button
          onClick={() => onStartPractice(module.id)}
          className="btn-warm flex-1"
        >
          📝 开始练习
        </button>
      </div>
    </div>
  );
};
