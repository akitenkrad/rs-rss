import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import rehypeRaw from 'rehype-raw';
import remarkGfm from 'remark-gfm';

const MemoForm = ({ 
    text, 
    onTextChange, 
    onSave, 
    onCancel, 
    onAskAI, 
    activeTab, 
    onTabChange,
    isGeneratingAI,
    CodeBlock 
}) => {
    return (
        <div className="memo-form">
            <div className="memo-tabs">
                <button 
                    className={`memo-tab ${activeTab === 'edit' ? 'active' : ''}`}
                    onClick={() => onTabChange('edit')}
                >
                    📝 編集
                </button>
                <button 
                    className={`memo-tab ${activeTab === 'preview' ? 'active' : ''}`}
                    onClick={() => onTabChange('preview')}
                >
                    👁️ プレビュー
                </button>
            </div>
            
            <div className="memo-tab-content">
                {activeTab === 'edit' ? (
                    <textarea
                        className="memo-textarea"
                        placeholder="メモをマークダウンで入力してください。AIに問い合わせる場合は、質問や依頼内容を書いてから「AIに問い合わせ」ボタンを押してください。&#10;&#10;例:&#10;# 重要なポイント&#10;- **Attention機構**の革新性&#10;- `Self-Attention`により並列処理が可能&#10;&#10;## 質問&#10;この論文の限界は何か？"
                        value={text}
                        onChange={(e) => onTextChange(e.target.value)}
                        rows={6}
                    />
                ) : (
                    <div className="memo-preview">
                        {text.trim() ? (
                            <ReactMarkdown
                                remarkPlugins={[remarkGfm]}
                                rehypePlugins={[rehypeHighlight, rehypeRaw]}
                                components={{
                                    code: CodeBlock,
                                    pre: ({ children }) => <>{children}</>
                                }}
                            >
                                {text}
                            </ReactMarkdown>
                        ) : (
                            <div className="preview-placeholder">
                                プレビューを表示するには、編集タブでマークダウンを入力してください。
                            </div>
                        )}
                    </div>
                )}
            </div>
            
            <div className="memo-form-buttons">
                <button 
                    className="save-memo-btn"
                    onClick={onSave}
                    disabled={!text.trim()}
                >
                    保存
                </button>
                <button 
                    className="cancel-memo-btn"
                    onClick={onCancel}
                >
                    キャンセル
                </button>
                <button 
                    className="ask-ai-btn"
                    onClick={onAskAI}
                    disabled={!text.trim() || isGeneratingAI}
                >
                    {isGeneratingAI ? '🤖 AIに問い合わせ中...' : '🤖 AIに問い合わせ'}
                </button>
            </div>
        </div>
    );
};

export default MemoForm;
