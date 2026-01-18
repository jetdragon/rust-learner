import React, { useEffect, useState } from 'react';
import { ModuleCard } from './components/ModuleCard';
import { AchievementsPanel } from './components/AchievementsPanel';
import { PracticeSession } from './components/PracticeSession';
import { modulesApi, practiceApi, achievementsApi, exportApi } from './api';
import type { LearningModule, PracticeQuestion, Achievement, PracticeResult } from './types';

function App() {
  const [modules, setModules] = useState<LearningModule[]>([]);
  const [achievements, setAchievements] = useState<Achievement[]>([]);
  const [showAchievements, setShowAchievements] = useState(false);
  const [practiceModule, setPracticeModule] = useState<string | null>(null);
  const [practiceQuestions, setPracticeQuestions] = useState<PracticeQuestion[]>([]);
  const [practiceResult, setPracticeResult] = useState<PracticeResult | undefined>();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [modulesData, achievementsData] = await Promise.all([
        modulesApi.getAll(),
        achievementsApi.getAll(),
      ]);
      setModules(modulesData);
      setAchievements(achievementsData);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateProgress = async (moduleId: string, taskType: string) => {
    try {
      await modulesApi.updateProgress(moduleId, taskType);
      await loadData(); // Refresh data
    } catch (error) {
      console.error('Failed to update progress:', error);
      alert('更新失败，请重试');
    }
  };

  const handleStartPractice = async (moduleId: string) => {
    try {
      const data = await practiceApi.getQuestions(moduleId);
      setPracticeModule(moduleId);
      setPracticeQuestions(data.questions);
      setPracticeResult(undefined);
    } catch (error) {
      console.error('Failed to load practice questions:', error);
      alert('加载练习题失败，请重试');
    }
  };

  const handleSubmitPractice = async (answers: number[]) => {
    if (!practiceModule) return;

    try {
      const result = await practiceApi.submit(practiceModule, answers);
      setPracticeResult(result);
      await loadData(); // Refresh data
    } catch (error) {
      console.error('Failed to submit practice:', error);
      alert('提交失败，请重试');
    }
  };

  const handleClosePractice = () => {
    setPracticeModule(null);
    setPracticeQuestions([]);
    setPracticeResult(undefined);
  };

  const handleExport = async () => {
    try {
      await exportApi.exportData();
    } catch (error) {
      console.error('Failed to export:', error);
      alert('导出失败，请重试');
    }
  };

  const completedModules = modules.filter(m => m.progress >= 95).length;
  const overallProgress = modules.length > 0 ? modules.reduce((sum, m) => sum + m.progress, 0) / modules.length : 0;

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="text-6xl mb-4 animate-pulse">🦀</div>
          <p className="text-warm-600 text-xl">加载中...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen">
      {/* Header */}
      <header className="bg-warm-600 text-white shadow-lg">
        <div className="container mx-auto px-4 py-6">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-3xl font-bold flex items-center gap-2">
                🦀 Rust 学习伴侣
              </h1>
              <p className="text-warm-100 mt-1">追踪你的 Rust 学习进度</p>
            </div>
            <div className="flex gap-4">
              <button
                onClick={() => setShowAchievements(true)}
                className="px-4 py-2 rounded-lg bg-warm-700 hover:bg-warm-800 transition-colors"
              >
                🏆 成就
              </button>
              <button
                onClick={handleExport}
                className="px-4 py-2 rounded-lg bg-warm-700 hover:bg-warm-800 transition-colors"
              >
                📤 导出
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Stats */}
      <div className="container mx-auto px-4 -mt-8">
        <div className="card-warm">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="text-center">
              <div className="text-4xl font-bold text-warm-500 mb-1">{overallProgress.toFixed(1)}%</div>
              <p className="text-warm-600">总体进度</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-warm-500 mb-1">{completedModules}</div>
              <p className="text-warm-600">已完成模块</p>
            </div>
            <div className="text-center">
              <div className="text-4xl font-bold text-warm-500 mb-1">{modules.length}</div>
              <p className="text-warm-600">总模块数</p>
            </div>
          </div>

          <div className="mt-6">
            <div className="flex justify-between text-sm mb-2">
              <span className="text-warm-700">总体完成度</span>
              <span className="font-semibold text-warm-800">{overallProgress.toFixed(0)}%</span>
            </div>
            <div className="w-full bg-warm-200 rounded-full h-4">
              <div
                className="progress-bar-warm h-4 rounded-full transition-all duration-500"
                style={{ width: `${overallProgress}%` }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Modules */}
      <div className="container mx-auto px-4 py-8">
        <h2 className="text-2xl font-bold text-warm-800 mb-6">学习模块</h2>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {modules.map((module) => (
            <ModuleCard
              key={module.id}
              module={module}
              onUpdateProgress={handleUpdateProgress}
              onStartPractice={handleStartPractice}
            />
          ))}
        </div>
      </div>

      {/* Footer */}
      <footer className="bg-warm-800 text-warm-200 py-8 mt-12">
        <div className="container mx-auto px-4 text-center">
          <p className="mb-2">🦀 Rust 学习伴侣 - Web 版</p>
          <p className="text-sm">追踪进度，激励学习，掌握 Rust</p>
        </div>
      </footer>

      {/* Modals */}
      {showAchievements && (
        <AchievementsPanel
          achievements={achievements}
          onClose={() => setShowAchievements(false)}
        />
      )}

      {practiceModule && practiceQuestions.length > 0 && (
        <PracticeSession
          moduleId={practiceModule}
          questions={practiceQuestions}
          onClose={handleClosePractice}
          onSubmit={handleSubmitPractice}
          result={practiceResult}
        />
      )}
    </div>
  );
}

export default App;
