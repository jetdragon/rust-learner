import React from 'react';
import type { PracticeQuestion } from '../types';

interface PracticeSessionProps {
  moduleId: string;
  questions: PracticeQuestion[];
  onClose: () => void;
  onSubmit: (answers: number[]) => void;
  result?: { score: number; correct_count: number; total_count: number };
}

export const PracticeSession: React.FC<PracticeSessionProps> = ({
  moduleId,
  questions,
  onClose,
  onSubmit,
  result,
}) => {
  const [currentQuestion, setCurrentQuestion] = React.useState(0);
  const [answers, setAnswers] = React.useState<number[]>(new Array(questions.length).fill(-1));
  const [showResult, setShowResult] = React.useState(!!result);

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
      onSubmit(answers);
    }
  };

  if (showResult && result) {
    const percentage = Math.round(result.score);
    const message =
      percentage >= 90
        ? '🎉 太棒了！'
        : percentage >= 70
        ? '💪 做得不错！'
        : percentage >= 50
        ? '📚 继续努力！'
        : '💪 不要放弃！';

    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="card-warm max-w-lg w-full mx-4 text-center" onClick={(e) => e.stopPropagation()}>
          <div className="text-6xl mb-4">📊</div>
          <h2 className="text-3xl font-bold text-warm-800 mb-2">练习结果</h2>
          <p className="text-xl text-warm-600 mb-4">{message}</p>

          <div className="text-5xl font-bold text-warm-500 mb-6">{percentage}%</div>

          <div className="bg-warm-50 rounded-lg p-4 mb-6">
            <p className="text-warm-700">
              正确 {result.correct_count} / {result.total_count} 题
            </p>
          </div>

          <button onClick={onClose} className="btn-warm w-full">
            继续学习
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
