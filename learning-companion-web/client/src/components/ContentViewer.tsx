import React from 'react';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import remarkGfm from 'remark-gfm';
import type { LearningModule } from '../types';
import type { LanguageTheme } from '../themes';

interface ContentViewerProps {
  module: LearningModule;
  contentType: string;
  onClose: () => void;
  theme?: LanguageTheme;
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

export const ContentViewer: React.FC<ContentViewerProps> = ({ module, contentType, onClose, theme }) => {
  const cardTheme = theme || { primary: '#dc5028', bg: '#fff7ed', text: '#7c2d12' };
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
        const data = await modulesApi.listExamples(module);
        setExamples(data.examples);
        setContent('');
      } else {
        // For other content types, fetch the content
        const data = await modulesApi.getContent(module, contentType);
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
      const data = await modulesApi.getExampleContent(module, filename);
      setContent(data.content);
      setSelectedExample(filename);
      setExamples([]);
      
      // Auto-detect if it's Rust code by extension
      if (filename.endsWith('.rs')) {
        // Force rust language for syntax highlighting
        setTimeout(() => {
          // This will be handled by the SyntaxHighlighter in render
        }, 0);
      }
    } catch (err) {
      setError('加载示例代码失败，请重试');
      console.error('Failed to load example:', err);
    } finally {
      setLoading(false);
    }
  };

  const cardStyle = {
    backgroundColor: cardTheme.bg,
    borderLeft: `4px solid ${cardTheme.primary}`,
    borderTop: `1px solid ${cardTheme.primary}33`,
    borderRight: `1px solid ${cardTheme.primary}33`,
    borderBottom: `1px solid ${cardTheme.primary}33`,
  };

  if (loading) {
    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] p-6" style={cardStyle} onClick={(e) => e.stopPropagation()}>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold" style={{ color: cardTheme.text }}>{getContentTypeName(contentType)}</h2>
            <button onClick={onClose} style={{ color: cardTheme.primary }} className="text-2xl hover:opacity-70">✕</button>
          </div>
          <div className="flex items-center justify-center py-20">
            <div style={{ color: cardTheme.text }} className="text-xl">加载中...</div>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
        <div className="rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] p-6" style={cardStyle} onClick={(e) => e.stopPropagation()}>
          <div className="flex justify-between items-center mb-6">
            <h2 className="text-2xl font-bold" style={{ color: cardTheme.text }}>{getContentTypeName(contentType)}</h2>
            <button onClick={onClose} style={{ color: cardTheme.primary }} className="text-2xl hover:opacity-70">✕</button>
          </div>
          <div className="text-center py-20">
            <div className="text-red-600 text-xl mb-4">❌ {error}</div>
            <button 
              onClick={loadContent} 
              className="px-4 py-2 rounded-lg text-white font-semibold hover:opacity-90 transition-opacity"
              style={{ backgroundColor: cardTheme.primary }}
            >
              重试
            </button>
          </div>
        </div>
      </div>
    );
  }

  const handleBackToExamples = () => {
    setSelectedExample(null);
    setContent('');
    setExamples([]); // Clear examples to force reload
    loadContent();
  };

  const handleClose = () => {
    if (selectedExample) {
      handleBackToExamples();
    } else {
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-[90vh] overflow-y-auto p-6" style={cardStyle} onClick={(e) => e.stopPropagation()}>
        <div className="flex items-center mb-6">
          {selectedExample && (
            <button
              onClick={handleBackToExamples}
              className="mr-3 transition-colors hover:opacity-70"
              style={{ color: cardTheme.primary }}
              aria-label="返回示例列表"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
            </button>
          )}
          <div className="flex-1">
            <h2 className="text-2xl font-bold" style={{ color: cardTheme.text }}>{getContentTypeName(contentType)}</h2>
            {selectedExample && <p style={{ color: cardTheme.primary }} className="text-sm">{selectedExample}</p>}
          </div>
          <button onClick={handleClose} style={{ color: cardTheme.primary }} className="text-2xl transition-colors hover:opacity-70">✕</button>
        </div>

        {content && selectedExample ? (
          // For example files (especially .rs), show as code directly, not markdown
          <div className="rounded-lg overflow-hidden my-4 bg-gray-900">
            <div className="bg-gray-800 text-gray-300 px-4 py-2 text-sm font-mono border-b border-gray-700">
              {selectedExample} (Rust)
            </div>
            <SyntaxHighlighter
              style={vscDarkPlus}
              language="rust"
              customStyle={{
                margin: 0,
                borderRadius: '0 0 0.5rem 0.5rem',
                fontSize: '0.875rem',
              }}
            >
              {content}
            </SyntaxHighlighter>
          </div>
        ) : content ? (
          // For non-example content (readme, exercises, project)
          <div className="prose prose-warm max-w-none prose-invert 
            prose-headings:text-warm-800 prose-h1:text-3xl prose-h1:font-bold prose-h1:mb-4
            prose-h2:text-2xl prose-h2:font-semibold prose-h2:mt-8 prose-h2:mb-3
            prose-h3:text-xl prose-h3:font-semibold prose-h3:mt-6 prose-h3:mb-2
            prose-p:text-warm-700 prose-p:mb-4
            prose-li:text-warm-700 prose-li:mb-1
            prose-strong:text-warm-800
            prose-blockquote:border-l-4 prose-blockquote:border-warm-300 prose-blockquote:pl-4 prose-blockquote:italic
            prose-table:border-warm-200
            prose-pre:bg-gray-900
          ">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                h1: ({node, ...props}) => <h1 className="text-4xl font-bold text-warm-800 mb-6 pb-3 border-b-2 border-warm-300" {...props} />,
                h2: ({node, ...props}) => <h2 className="text-3xl font-semibold text-warm-700 mt-8 mb-4 pb-2 border-b border-warm-200" {...props} />,
                h3: ({node, ...props}) => <h3 className="text-2xl font-semibold text-warm-600 mt-6 mb-3" {...props} />,
                h4: ({node, ...props}) => <h4 className="text-xl font-semibold text-warm-600 mt-4 mb-2" {...props} />,
                h5: ({node, ...props}) => <h5 className="text-lg font-semibold text-warm-600 mt-3 mb-1" {...props} />,
                h6: ({node, ...props}) => <h6 className="text-base font-semibold text-warm-600 mt-2 mb-1" {...props} />,
                p: ({node, ...props}) => <p className="text-warm-700 mb-4 leading-relaxed" {...props} />,
                ul: ({node, ...props}) => <ul className="list-disc list-inside mb-4 ml-4 text-warm-700 space-y-1" {...props} />,
                ol: ({node, ...props}) => <ol className="list-decimal list-inside mb-4 ml-4 text-warm-700 space-y-1" {...props} />,
                li: ({node, ...props}) => <li className="mb-1" {...props} />,
                a: ({node, ...props}) => <a className="text-warm-600 hover:text-warm-800 underline-offset-2 hover:underline" {...props} />,
                blockquote: ({node, ...props}) => (
                  <blockquote className="border-l-4 border-warm-300 pl-4 py-2 my-4 bg-warm-50 italic rounded-r-lg" {...props} />
                ),
                table: ({node, ...props}) => (
                  <div className="overflow-x-auto my-6">
                    <table className="w-full border-collapse border border-warm-200 rounded-lg overflow-hidden" {...props} />
                  </div>
                ),
                thead: ({node, ...props}) => <thead className="bg-warm-100" {...props} />,
                tbody: ({node, ...props}) => <tbody {...props} />,
                tr: ({node, ...props}) => <tr className="border-b border-warm-100 hover:bg-warm-50" {...props} />,
                th: ({node, ...props}) => <th className="border border-warm-200 px-4 py-3 text-left font-semibold text-warm-800" {...props} />,
                td: ({node, ...props}) => <td className="border border-warm-200 px-4 py-3 text-warm-700" {...props} />,
                hr: ({node, ...props}) => <hr className="border-t border-warm-200 my-8" {...props} />,
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
        ) : examples.length > 0 ? (
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
          <div className="text-center py-10">
            <p className="text-warm-600">暂无内容</p>
          </div>
        )}
      </div>
    </div>
  );
};