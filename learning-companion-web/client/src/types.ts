export interface LearningModule {
  id: string;
  name: string;
  language: string;  // Language identifier: "rust", "python", "go"
  has_readme: boolean;
  has_exercises: boolean;
  has_tests: boolean;
  has_checklist: boolean;
  progress: number;
  tasks: ModuleTasks;
}

export interface ModuleTasks {
  concept: boolean;
  examples: boolean;
  exercises: boolean;
  project: boolean;
  checklist: boolean;
}

export interface PracticeQuestion {
  id: number;
  question: string;
  options: string[];
  correct_answer: string;
}

export interface Achievement {
  name: string;
  description: string;
  unlocked: boolean;
}

export interface PracticeResult {
  score: number;
  correct_count: number;
  total_count: number;
}
