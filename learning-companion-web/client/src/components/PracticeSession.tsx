import React, { useEffect } from 'react';
import type { PracticeQuestion } from '../types';

interface PracticeSessionProps {
  moduleId: string;
  questions: PracticeQuestion[];
  onClose: () => void;
  onSubmit: (answers: number[]) => void;
  result?: { score: number; correct_count: number; total_count: number };
}

export const PracticeSession: React.FC<PracticeSessionProps> = ({
  moduleId: _moduleId,
  questions,
  onClose,
  onSubmit,
  result,
}) => {
  const [currentQuestion, setCurrentQuestion] = React.useState(0);
  const [answers, setAnswers] = React.useState<number[]>(new Array(questions.length).fill(-1));
  const [showResult, setShowResult] = React.useState(false);

  const handleAnswer = (questionIndex: number, answerIndex: number) => {
    const newAnswers = [...answers];
    newAnswers[questionIndex] = answerIndex;
    setAnswers(newAnswers);
  };

  const handlePrevious = () => {
    if (currentQuestion > 0) setCurrentQuestion(currentQuestion - 1);
  };

  const handleNext = () => {
    if (currentQuestion < questions.length - 1) {
      setCurrentQuestion(currentQuestion + 1);
    } else {
      // Submit answers when on the last question
      onSubmit(answers);
      setShowResult(true);
    }
  };

  useEffect(() => {
    if (result && showResult) {
      // Auto-close after 5 seconds
      const timer = setTimeout(() => {
        console.log('Auto-closing practice session...');
        onClose();
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [result, showResult, onClose]);

  if (showResult && result) {
    const percentage = Math.round(result.score);
    const getPerformanceMessage = () => {
      if (percentage >= 90) return { emoji: '🎉', message: '太棒了！满分通过！', color: 'text-green-600' };
      if (percentage >= 70) return { emoji: '💪', message: '做得不错！', color: 'text-blue-600' };
      if (percentage >= 50) return { emoji: '📚', message: '继续努力，你可以的！', color: 'text-orange-600' };
      return { emoji: '📖', message: '建议复习后再试一次', color: 'text-red-600' };
    };

    const performance = getPerformanceMessage();

    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="card-warm max-w-2xl w-full mx-4 text-center" onClick={(e) => e.stopPropagation()}>
          <div className="text-6xl mb-4">{performance.emoji}</div>
          <h2 className="text-3xl font-bold text-warm-800 mb-2">练习结果</h2>
          <p className={`text-xl mb-4 font-semibold ${performance.color}`}>{performance.message}</p>

          <div className={`text-5xl font-bold mb-6 ${performance.color}`}>{percentage}%</div>

          <div className="bg-warm-50 rounded-lg p-6 mb-6 text-left">
            <h3 className="text-lg font-semibold text-warm-800 mb-3">答题统计</h3>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-warm-700">正确题数：</span>
                <span className="font-bold text-green-600">{result.correct_count} 题</span>
              </div>
              <div className="flex justify-between">
                <span className="text-warm-700">错误题数：</span>
                <span className="font-bold text-red-600">{result.total_count - result.correct_count} 题</span>
              </div>
              <div className="flex justify-between">
                <span className="text-warm-700">总题数：</span>
                <span className="font-bold">{result.total_count} 题</span>
              </div>
              <div className="mt-4 pt-4 border-t border-warm-200">
                <div className="flex justify-between items-center">
                  <span className="text-warm-700">正确率：</span>
                  <span className={`text-2xl font-bold ${performance.color}`}>{percentage}%</span>
                </div>
              </div>
            </div>
          </div>

          <div className="text-warm-600 text-sm mb-4">
            将在 5 秒后自动关闭...
          </div>

          <button onClick={onClose} className="btn-warm w-full">
            立即关闭
          </button>
        </div>
      </div>
    );
  }

  const question = questions[currentQuestion];

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="card-warm max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-4">
          <div>
            <h2 className="text-2xl font-bold text-warm-800">📝 练习模式</h2>
            <p className="text-warm-600">
              题目 {currentQuestion + 1} / {questions.length}
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-warm-400 hover:text-warm-600 text-2xl"
          >
            ✕
          </button>
        </div>

        <div className="mb-6">
          <h3 className="text-xl font-semibold text-warm-800 mb-4">{question.question}</h3>

          <div className="space-y-3">
            {question.options.map((option, index) => (
              <button
                key={index}
                onClick={() => handleAnswer(currentQuestion, index)}
                className={`w-full p-4 rounded-lg border-2 text-left transition-all ${
                  answers[currentQuestion] === index
                    ? 'bg-warm-500 text-white border-warm-600'
                    : 'bg-white border-warm-300 text-warm-800 hover:bg-warm-50 hover:border-warm-400'
                }`}
              >
                <span className="font-semibold mr-2">{String.fromCharCode(65 + index)}.</span>
                {option}
              </button>
            ))}
          </div>
        </div>

        <div className="flex justify-between">
          <button
            onClick={handlePrevious}
            disabled={currentQuestion === 0}
            className="px-6 py-3 rounded-lg font-semibold transition-all duration-200
                   bg-warm-300 hover:bg-warm-400 disabled:opacity-50 disabled:cursor-not-allowed
                   text-warm-800"
          >
            ← 上一题
          </button>
          <button
            onClick={handleNext}
            className="btn-warm"
          >
            {currentQuestion === questions.length - 1 ? '提交答案' : '下一题 →'}
          </button>
        </div>
      </div>
    </div>
  );
};
