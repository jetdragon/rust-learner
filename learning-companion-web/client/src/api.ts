import axios from 'axios';
import type { LearningModule, PracticeQuestion, Achievement, PracticeResult } from './types';

const API_BASE = '/api';

export const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const modulesApi = {
  getAll: async (): Promise<LearningModule[]> => {
    const response = await api.get<LearningModule[]>('/modules');
    return response.data;
  },

  updateProgress: async (moduleId: string, taskType: string): Promise<{ success: boolean; mastery: number }> => {
    const response = await api.post(`/modules/${moduleId}/progress`, { task_type: taskType });
    return response.data;
  },

  getContent: async (module: LearningModule, contentType: string): Promise<{ content: string }> => {
    const response = await api.get<{ content: string }>(`/modules/${module.language}/${module.id}/content/${contentType}`);
    return response.data;
  },

  listExamples: async (module: LearningModule): Promise<{ examples: string[] }> => {
    const response = await api.get<{ examples: string[] }>(`/modules/${module.language}/${module.id}/examples`);
    return response.data;
  },

  getExampleContent: async (module: LearningModule, filename: string): Promise<{ content: string }> => {
    const response = await api.get<{ content: string }>(`/modules/${module.language}/${module.id}/examples/${filename}`);
    return response.data;
  },
};

export const practiceApi = {
  getQuestions: async (moduleId: string): Promise<{ questions: PracticeQuestion[] }> => {
    const response = await api.get<{ questions: PracticeQuestion[] }>(`/practice/${moduleId}`);
    return response.data;
  },

  submit: async (moduleId: string, answers: number[]): Promise<PracticeResult> => {
    const response = await api.post(`/practice/submit/${moduleId}`, { answers });
    return response.data;
  },
};

export const achievementsApi = {
  getAll: async (): Promise<Achievement[]> => {
    const response = await api.get<Achievement[]>('/achievements');
    return response.data;
  },
};

export const exportApi = {
  exportData: async () => {
    const response = await api.get('/export', {
      responseType: 'blob',
    });
    const url = window.URL.createObjectURL(new Blob([response.data], { type: 'application/json' }));
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `learning-companion-export-${new Date().toISOString().split('T')[0]}.json`);
    document.body.appendChild(link);
    link.click();
    link.remove();
  },
};
