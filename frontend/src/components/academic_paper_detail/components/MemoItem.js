import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import rehypeRaw from 'rehype-raw';
import remarkGfm from 'remark-gfm';
import AskToAgentForm from './AskToAgentForm';

const MemoItem = ({
    memo,
    isEditing,
    editingText,
    activeViewTab,
    activeEditTab,
    isGeneratingEdit,
    onStartEdit,
    onEdit,
    onCancelEdit,
    onDelete,
    onTextChange,
    onTabChange,
    onGenerateEdit,
    askAgentQuery,
    isAskingAgent,
    showAskAgentForm,
    onAskAgentQueryChange,
    onAskToAgent,
    onShowAskAgentForm,
    onCancelAskAgent,
    CodeBlock
}) => {
    return (
        <div className={`memo-item ${memo.type}`}>
            <div className="memo-header">
                <div className="memo-date">
                    {new Date(memo.createdAt).toLocaleString('ja-JP')}
                </div>
                <div className="memo-actions">
                    <button 
                        className="edit-memo-btn"
                        onClick={() => onStartEdit(memo.id)}
                    >
                        編集
                    </button>
                    <button 
                        className="delete-memo-btn"
                        onClick={() => {
                            if (window.confirm('このメモを削除しますか？この操作は取り消せません。')) {
                                onDelete(memo.id);
                            }
                        }}
                    >
                        削除
                    </button>
                </div>
            </div>
            
            {isEditing ? (
                <div className="memo-edit-form">
                    <div className="memo-tabs">
                        <button 
                            className={`memo-tab ${(activeEditTab || 'edit') === 'edit' ? 'active' : ''}`}
                            onClick={() => onTabChange(memo.id, 'edit', 'edit')}
                        >
                            📝 編集
                        </button>
                        <button 
                            className={`memo-tab ${activeEditTab === 'preview' ? 'active' : ''}`}
                            onClick={() => onTabChange(memo.id, 'edit', 'preview')}
                        >
                            👁️ プレビュー
                        </button>
                    </div>
                    
                    <div className="memo-tab-content">
                        {(activeEditTab || 'edit') === 'edit' ? (
                            <textarea
                                className="memo-edit-textarea"
                                value={editingText !== undefined ? editingText : memo.text}
                                onChange={(e) => onTextChange(memo.id, e.target.value)}
                                rows={6}
                                id={`edit-textarea-${memo.id}`}
                                onKeyDown={(e) => {
                                    if (e.key === 'Enter' && e.ctrlKey) {
                                        const currentText = editingText !== undefined ? editingText : memo.text;
                                        onEdit(memo.id, currentText);
                                    }
                                }}
                            />
                        ) : (
                            <div className="memo-preview">
                                <ReactMarkdown
                                    remarkPlugins={[remarkGfm]}
                                    rehypePlugins={[rehypeHighlight, rehypeRaw]}
                                    components={{
                                        code: CodeBlock,
                                        pre: ({ children }) => <>{children}</>
                                    }}
                                >
                                    {editingText !== undefined ? editingText : memo.text}
                                </ReactMarkdown>
                            </div>
                        )}
                    </div>
                    
                    <div className="memo-edit-buttons">
                        <button 
                            className="save-edit-btn"
                            onClick={() => {
                                const currentText = editingText !== undefined ? editingText : memo.text;
                                onEdit(memo.id, currentText);
                            }}
                        >
                            保存
                        </button>
                        <button 
                            className="cancel-edit-btn"
                            onClick={() => onCancelEdit(memo.id)}
                        >
                            キャンセル
                        </button>
                    </div>
                </div>
            ) : (
                <div className="memo-view">
                    <div className="memo-tabs">
                        <button 
                            className={`memo-tab ${(activeViewTab || 'preview') === 'preview' ? 'active' : ''}`}
                            onClick={() => onTabChange(memo.id, 'view', 'preview')}
                        >
                            👁️ プレビュー
                        </button>
                        <button 
                            className={`memo-tab ${activeViewTab === 'source' ? 'active' : ''}`}
                            onClick={() => onTabChange(memo.id, 'view', 'source')}
                        >
                            📄 ソース
                        </button>
                    </div>
                    
                    <div className="memo-tab-content">
                        {(activeViewTab || 'preview') === 'preview' ? (
                            <div className="memo-content">
                                <ReactMarkdown
                                    remarkPlugins={[remarkGfm]}
                                    rehypePlugins={[rehypeHighlight, rehypeRaw]}
                                    components={{
                                        code: CodeBlock,
                                        pre: ({ children }) => <>{children}</>
                                    }}
                                >
                                    {memo.text}
                                </ReactMarkdown>
                            </div>
                        ) : (
                            <div className="memo-source">
                                <pre><code>{memo.text}</code></pre>
                            </div>
                        )}
                    </div>
                    
                    {/* Ask-to-agent機能 */}
                    <div className="memo-ask-agent-section">
                        <AskToAgentForm
                            memoId={memo.id}
                            query={askAgentQuery}
                            onQueryChange={(value) => onAskAgentQueryChange(memo.id, value)}
                            onSubmit={() => onAskToAgent(memo.id)}
                            onCancel={() => onCancelAskAgent(memo.id)}
                            isSubmitting={isAskingAgent}
                            showForm={showAskAgentForm}
                            onShowForm={() => onShowAskAgentForm(memo.id)}
                        />
                    </div>
                </div>
            )}
        </div>
    );
};

export default MemoItem;
