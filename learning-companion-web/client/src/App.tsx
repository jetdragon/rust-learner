import { useEffect, useState, useMemo } from 'react';
import { ModuleCard } from './components/ModuleCard';
import { AchievementsPanel } from './components/AchievementsPanel';
import { PracticeSession } from './components/PracticeSession';
import { ContentViewer } from './components/ContentViewer';
import { modulesApi, practiceApi, achievementsApi, exportApi } from './api';
import { getLanguageTheme, LANGUAGE_THEMES } from './themes';
import type { LearningModule, PracticeQuestion, Achievement, PracticeResult } from './types';

function App() {
  const [modules, setModules] = useState<LearningModule[]>([]);
  const [achievements, setAchievements] = useState<Achievement[]>([]);
  const [showAchievements, setShowAchievements] = useState(false);
  const [practiceModule, setPracticeModule] = useState<string | null>(null);
  const [practiceQuestions, setPracticeQuestions] = useState<PracticeQuestion[]>([]);
  const [practiceResult, setPracticeResult] = useState<PracticeResult | undefined>();
  const [loading, setLoading] = useState(true);
  const [showContentViewer, setShowContentViewer] = useState(false);
  const [currentModule, setCurrentModule] = useState<LearningModule | null>(null);
  const [currentContentType, setCurrentContentType] = useState<string>('');

  useEffect(() => {
    loadData();
  }, []);

  // Group modules by language
  const modulesByLanguage = useMemo(() => {
    const grouped: Record<string, LearningModule[]> = {};
    
    modules.forEach(module => {
      const lang = module.language || 'unknown';
      if (!grouped[lang]) {
        grouped[lang] = [];
      }
      grouped[lang].push(module);
    });
    
    // Sort by language name and module ID
    const sortedGroups: Record<string, LearningModule[]> = {};
    Object.keys(grouped).sort().forEach(lang => {
      sortedGroups[lang] = grouped[lang].sort((a, b) => a.id.localeCompare(b.id));
    });
    
    return sortedGroups;
  }, [modules]);

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

  const handleViewContent = (moduleId: string, contentType: string) => {
    const module = modules.find(m => m.id === moduleId);
    if (module) {
      setCurrentModule(module);
      setCurrentContentType(contentType);
      setShowContentViewer(true);
    }
  };

  const handleCloseContentViewer = () => {
    setShowContentViewer(false);
    setCurrentModule(null);
    setCurrentContentType('');
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
                🦀🐍🐹 多语言学习伴侣
              </h1>
              <p className="text-warm-100 mt-1">追踪你的多语言学习进度</p>
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

      {/* Modules - Grouped by Language */}
      <div className="container mx-auto px-4 py-8">
        <h2 className="text-3xl font-bold text-warm-800 mb-8">学习模块</h2>
        
        {Object.entries(modulesByLanguage).map(([lang, langModules]) => {
          const theme = LANGUAGE_THEMES[lang] || getLanguageTheme(lang);
          
          return (
            <div key={lang} className="mb-12">
              {/* Language Group Header */}
              <div 
                className="mb-6 px-6 py-4 rounded-lg shadow-sm flex items-center gap-3"
                style={{
                  borderLeft: `6px solid ${theme.primary}`,
                  backgroundColor: theme.bg
                }}
              >
                <span className="text-4xl">{theme.emoji}</span>
                <div>
                  <h3 
                    className="text-2xl font-bold"
                    style={{ color: theme.primary }}
                  >
                    {theme.name} 模块
                  </h3>
                  <p className="text-sm" style={{ color: theme.text, opacity: 0.7 }}>
                    {langModules.length} 个模块
                  </p>
                </div>
              </div>

              {/* Module Cards Grid */}
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {langModules.map((module) => (
                  <ModuleCard
                    key={module.id}
                    module={module}
                    theme={theme}
                    onUpdateProgress={handleUpdateProgress}
                    onStartPractice={handleStartPractice}
                    onViewContent={handleViewContent}
                  />
                ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Footer */}
      <footer className="bg-warm-800 text-warm-200 py-8 mt-12">
        <div className="container mx-auto px-4 text-center">
          <p className="mb-2">🦀🐍🐹 多语言学习伴侣 - Web 版</p>
          <p className="text-sm">追踪进度，激励学习，掌握多种编程语言</p>
        </div>
      </footer>

      {/* Modals */}
      {showAchievements && (
        <AchievementsPanel
          achievements={achievements}
          onClose={() => setShowAchievements(false)}
        />
      )}

      {showContentViewer && currentModule && (
        <ContentViewer
          module={currentModule}
          contentType={currentContentType}
          onClose={handleCloseContentViewer}
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
