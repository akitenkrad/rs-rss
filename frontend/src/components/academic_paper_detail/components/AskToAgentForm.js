
const AskToAgentForm = ({ 
    memoId, 
    query, 
    onQueryChange, 
    onSubmit, 
    onCancel, 
    isSubmitting,
    showForm,
    onShowForm 
}) => {
    if (!showForm) {
        return (
            <button 
                className="ask-agent-btn"
                onClick={onShowForm}
            >
                🤖 このメモについてエージェントに質問
            </button>
        );
    }

    return (
        <div className="ask-agent-form">
            <textarea
                className="ask-agent-textarea"
                placeholder="このメモについて質問を入力してください..."
                value={query || ''}
                onChange={(e) => onQueryChange(e.target.value)}
                rows={3}
            />
            <div className="ask-agent-buttons">
                <button 
                    className="submit-question-btn"
                    onClick={onSubmit}
                    disabled={isSubmitting || !query?.trim()}
                >
                    {isSubmitting ? '🤖 質問中...' : '✅ 質問を送信'}
                </button>
                <button 
                    className="cancel-ask-btn"
                    onClick={onCancel}
                >
                    キャンセル
                </button>
            </div>
        </div>
    );
};

export default AskToAgentForm;
