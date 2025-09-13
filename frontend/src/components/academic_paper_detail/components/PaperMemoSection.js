import MemoForm from './MemoForm';
import MemoItem from './MemoItem';

const PaperMemoSection = ({
    memos,
    isAddingMemo,
    newMemoText,
    addMemoActiveTab,
    editingMemoId,
    editingMemoTexts,
    editMemoActiveTabs,
    viewMemoActiveTabs,
    isGeneratingLlmMemo,
    isGeneratingEditMemo,
    askAgentQuery,
    isAskingAgent,
    showAskAgentForm,
    onStartAddMemo,
    onAddMemo,
    onCancelAddMemo,
    onNewMemoTextChange,
    onAddMemoTabChange,
    onGenerateLlmMemo,
    onStartEditMemo,
    onEditMemo,
    onCancelEditMemo,
    onDeleteMemo,
    onEditingTextChange,
    onTabChange,
    onGenerateEditMemo,
    onAskAgentQueryChange,
    onAskToAgent,
    onShowAskAgentForm,
    onCancelAskAgent,
    CodeBlock
}) => {
    return (
        <section className="memos-section">
            <h2>Notes & Memos</h2>
            
            {/* メモ追加エリア */}
            <div className="memo-add-section">
                <div className="memo-add-buttons">
                    <button 
                        className="add-memo-btn"
                        onClick={onStartAddMemo}
                    >
                        ✏️ メモを追加
                    </button>
                </div>

                {/* メモ追加フォーム */}
                {isAddingMemo && (
                    <MemoForm
                        text={newMemoText}
                        onTextChange={onNewMemoTextChange}
                        onSave={onAddMemo}
                        onCancel={onCancelAddMemo}
                        onAskAI={onGenerateLlmMemo}
                        activeTab={addMemoActiveTab}
                        onTabChange={onAddMemoTabChange}
                        isGeneratingAI={isGeneratingLlmMemo}
                        CodeBlock={CodeBlock}
                    />
                )}
            </div>

            {/* 既存メモ一覧 */}
            <div className="memos-list">
                {memos.length === 0 ? (
                    <div className="no-memos">
                        まだメモがありません。上記のボタンからメモを追加してください。
                    </div>
                ) : (
                    memos.map((memo) => (
                        <MemoItem
                            key={memo.id}
                            memo={memo}
                            isEditing={editingMemoId === memo.id}
                            editingText={editingMemoTexts[memo.id]}
                            activeViewTab={viewMemoActiveTabs[memo.id]}
                            activeEditTab={editMemoActiveTabs[memo.id]}
                            isGeneratingEdit={isGeneratingEditMemo[memo.id]}
                            onStartEdit={onStartEditMemo}
                            onEdit={onEditMemo}
                            onCancelEdit={onCancelEditMemo}
                            onDelete={onDeleteMemo}
                            onTextChange={onEditingTextChange}
                            onTabChange={onTabChange}
                            onGenerateEdit={onGenerateEditMemo}
                            askAgentQuery={askAgentQuery[memo.id]}
                            isAskingAgent={isAskingAgent[memo.id]}
                            showAskAgentForm={showAskAgentForm[memo.id]}
                            onAskAgentQueryChange={onAskAgentQueryChange}
                            onAskToAgent={onAskToAgent}
                            onShowAskAgentForm={onShowAskAgentForm}
                            onCancelAskAgent={onCancelAskAgent}
                            CodeBlock={CodeBlock}
                        />
                    ))
                )}
            </div>
        </section>
    );
};

export default PaperMemoSection;
