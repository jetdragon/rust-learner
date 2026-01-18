import React from 'react';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import remarkGfm from 'remark-gfm';
import type { LearningModule } from '../types';

interface ContentViewerProps {
  module: LearningModule;
  contentType: string;
  onClose: () => void;
}

const getContentTypeName = (contentType: string): string => {
  const names: Record<string, string> = {
    'readme': '📖 README - 模块说明',
    'exercises': '✏️ 练习题',
    'project': '📦 综合练习项目',
    'examples': '💻 代码示例',
  };
  return names[contentType] || '📄 内容';
};

export const ContentViewer: React.FC<ContentViewerProps> = ({ module, contentType, onClose }) => {
  const [content, setContent] = React.useState<string>('');
  const [loading, setLoading] = React.useState<boolean>(true);
  const [error, setError] = React.useState<string | null>(null);
  const [examples, setExamples] = React.useState<string[]>([]);
  const [selectedExample, setSelectedExample] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadContent();
  }, [module.id, contentType]);

  const loadContent = async () => {
    setLoading(true);
    setError(null);

    try {
      const { modulesApi } = await import('../api');

      if (contentType === 'examples') {
        // For examples, list all example files
        const data = await modulesApi.listExamples(module.id);
        setExamples(data.examples);
        setContent('');
      } else {
        // For other content types, fetch the content
        const data = await modulesApi.getContent(module.id, contentType);
        setContent(data.content);
        setExamples([]);
      }
    } catch (err) {
      setError('加载内容失败，请重试');
      console.error('Failed to load content:', err);
    } finally {
      setLoading(false);
    }
  };

  const loadExampleContent = async (filename: string) => {
    setLoading(true);
    setError(null);

    try {
      const { modulesApi } = await import('../api');
      const data = await modulesApi.getExampleContent(module.id, filename);
      setContent(data.content);
      setSelectedExample(filename);
      setExamples([]);
    } catch (err) {
      setError('加载示例代码失败，请重试');
      console.error('Failed to load example:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="card-warm max-w-4xl w-full mx-4 max-h-[90vh]" onClick={(e) => e.stopPropagation()}>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-warm-800">{getContentTypeName(contentType)}</h2>
            <button onClick={onClose} className="text-warm-400 hover:text-warm-600 text-2xl">✕</button>
          </div>
          <div className="flex items-center justify-center py-20">
            <div className="text-warm-600 text-xl">加载中...</div>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="card-warm max-w-4xl w-full mx-4 max-h-[90vh]" onClick={(e) => e.stopPropagation()}>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold text-warm-800">{getContentTypeName(contentType)}</h2>
            <button onClick={onClose} className="text-warm-400 hover:text-warm-600 text-2xl">✕</button>
          </div>
          <div className="text-center py-20">
            <div className="text-red-600 text-xl mb-4">❌ {error}</div>
            <button onClick={loadContent} className="btn-warm">重试</button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="card-warm max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-6">
          <div>
            <h2 className="text-2xl font-bold text-warm-800">{getContentTypeName(contentType)}</h2>
            {selectedExample && <p className="text-warm-600">{selectedExample}</p>}
          </div>
          <button onClick={onClose} className="text-warm-400 hover:text-warm-600 text-2xl">✕</button>
        </div>

        {examples.length > 0 ? (
          <div>
            <h3 className="text-xl font-semibold text-warm-700 mb-4">选择示例文件：</h3>
            <div className="space-y-2">
              {examples.map((example) => (
                <button
                  key={example}
                  onClick={() => loadExampleContent(example)}
                  className="w-full p-3 text-left bg-warm-50 hover:bg-warm-100 rounded-lg transition-colors text-warm-800 font-mono"
                >
                  📄 {example}
                </button>
              ))}
            </div>
          </div>
        ) : (
          <div className="prose prose-warm max-w-none prose-headings:text-warm-800 prose-p:text-warm-700 prose-li:text-warm-700 prose-strong:text-warm-800">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                code(props) {
                  const { inline, className, children } = props as any;
                  const match = /language-(\w+)/.exec(className || '');
                  if (!inline && match) {
                    return (
                      <div className="rounded-lg overflow-hidden my-4">
                        <div className="bg-gray-800 text-gray-300 px-4 py-2 text-sm font-mono border-b border-gray-700">
                          {match[1]}
                        </div>
                        <SyntaxHighlighter
                          style={vscDarkPlus}
                          language={match[1]}
                          customStyle={{
                            margin: 0,
                            borderRadius: '0 0 0.5rem 0.5rem',
                            fontSize: '0.875rem',
                          }}
                        >
                          {String(children).replace(/\n$/, '')}
                        </SyntaxHighlighter>
                      </div>
                    );
                  }
                  return (
                    <code className="bg-gray-100 px-1 py-0.5 rounded text-sm font-mono text-red-600 font-semibold">
                      {String(children).replace(/\n$/, '')}
                    </code>
                  );
                },
              }}
            >
              {content}
            </ReactMarkdown>
          </div>
        )}
      </div>
    </div>
  );
};