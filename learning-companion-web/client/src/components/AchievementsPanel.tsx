import React from 'react';
import type { Achievement } from '../types';

interface AchievementsPanelProps {
  achievements: Achievement[];
  onClose: () => void;
}

export const AchievementsPanel: React.FC<AchievementsPanelProps> = ({ achievements, onClose }) => {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="card-warm max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-3xl font-bold text-warm-800">🏆 成就系统</h2>
          <button
            onClick={onClose}
            className="text-warm-400 hover:text-warm-600 text-2xl"
          >
            ✕
          </button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {achievements.map((achievement) => (
            <div
              key={achievement.name}
              className={`p-4 rounded-lg border-2 transition-all ${
                achievement.unlocked
                  ? 'bg-warm-100 border-warm-400'
                  : 'bg-warm-50 border-warm-200 opacity-60'
              }`}
            >
              <div className="flex items-center gap-3 mb-2">
                <div className={`text-3xl ${achievement.unlocked ? '' : 'grayscale opacity-50'}`}>
                  {achievement.unlocked ? '🏅' : '🔒'}
                </div>
                <h3 className="font-bold text-warm-800">{achievement.description}</h3>
              </div>
              {achievement.unlocked && (
                <p className="text-sm text-warm-600 mt-2">已解锁</p>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
