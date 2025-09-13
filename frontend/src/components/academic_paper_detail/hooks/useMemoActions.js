import { useState } from 'react';
import { handleApiError, paperNotesApi } from '../../api/Api';

export const useMemoActions = (paperId, memos, setMemos, setError) => {
    // メモ関連のstate
    const [isAddingMemo, setIsAddingMemo] = useState(false);
    const [editingMemoId, setEditingMemoId] = useState(null);
    const [newMemoText, setNewMemoText] = useState('');
    const [isGeneratingLlmMemo, setIsGeneratingLlmMemo] = useState(false);
    
    // Ask-to-agent関連のstate
    const [askAgentQuery, setAskAgentQuery] = useState({});
    const [isAskingAgent, setIsAskingAgent] = useState({});
    const [showAskAgentForm, setShowAskAgentForm] = useState({});
    
    // タブ管理用のstate
    const [addMemoActiveTab, setAddMemoActiveTab] = useState('edit');
    const [editMemoActiveTabs, setEditMemoActiveTabs] = useState({});
    const [viewMemoActiveTabs, setViewMemoActiveTabs] = useState({});
    const [isGeneratingEditMemo, setIsGeneratingEditMemo] = useState({});
    
    // 編集中のメモ内容を管理するstate
    const [editingMemoTexts, setEditingMemoTexts] = useState({});

    // メモ関連の関数
    const handleAddMemo = async () => {
        if (newMemoText.trim()) {
            try {
                if (process.env.NODE_ENV === 'development') {
                    const isLlmGenerated = newMemoText.includes('--- AIによる追加情報');
                    const newMemo = {
                        id: Date.now(),
                        text: newMemoText.trim(),
                        type: isLlmGenerated ? 'llm' : 'manual',
                        createdAt: new Date().toISOString(),
                        updatedAt: new Date().toISOString()
                    };
                    setMemos([...memos, newMemo]);
                } else {
                    const noteData = {
                        paper_id: paperId,
                        text: newMemoText.trim(),
                        note_timestamp: new Date().toISOString().split('T')[0]
                    };
                    const response = await paperNotesApi.create(noteData);
                    
                    const newMemo = {
                        id: response.paper_note.paper_note_id,
                        text: response.paper_note.text,
                        type: 'manual',
                        createdAt: response.paper_note.note_timestamp,
                        updatedAt: response.paper_note.note_timestamp
                    };
                    setMemos([...memos, newMemo]);
                }
                
                setNewMemoText('');
                setIsAddingMemo(false);
                setAddMemoActiveTab('edit');
            } catch (err) {
                console.error('メモ作成エラー:', err);
                const apiError = handleApiError(err);
                setError(apiError.message || 'メモの作成に失敗しました');
            }
        }
    };

    const handleEditMemo = async (id, newText) => {
        try {
            if (process.env.NODE_ENV === 'development') {
                setMemos(memos.map(memo => 
                    memo.id === id 
                        ? { ...memo, text: newText, updatedAt: new Date().toISOString() }
                        : memo
                ));
            } else {
                const updateData = {
                    paper_note_id: id,
                    paper_id: paperId,
                    text: newText,
                    note_timestamp: new Date().toISOString().split('T')[0]
                };
                
                await paperNotesApi.update(updateData);
                
                setMemos(memos.map(memo => 
                    memo.id === id 
                        ? { ...memo, text: newText, updatedAt: new Date().toISOString() }
                        : memo
                ));
            }
            
            setEditingMemoId(null);
            setViewMemoActiveTabs(prev => ({ ...prev, [id]: 'preview' }));
            setEditingMemoTexts(prev => {
                const newTexts = { ...prev };
                delete newTexts[id];
                return newTexts;
            });
        } catch (err) {
            console.error('メモ更新エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || 'メモの更新に失敗しました');
        }
    };

    const handleDeleteMemo = async (id) => {
        try {
            if (process.env.NODE_ENV === 'development') {
                setMemos(memos.filter(memo => memo.id !== id));
            } else {
                await paperNotesApi.delete(id);
                setMemos(memos.filter(memo => memo.id !== id));
            }
            
            // タブ状態をクリーンアップ
            setEditMemoActiveTabs(prev => {
                const newTabs = { ...prev };
                delete newTabs[id];
                return newTabs;
            });
            setViewMemoActiveTabs(prev => {
                const newTabs = { ...prev };
                delete newTabs[id];
                return newTabs;
            });
            setIsGeneratingEditMemo(prev => {
                const newTabs = { ...prev };
                delete newTabs[id];
                return newTabs;
            });
            setEditingMemoTexts(prev => {
                const newTexts = { ...prev };
                delete newTexts[id];
                return newTexts;
            });
        } catch (err) {
            console.error('メモ削除エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || 'メモの削除に失敗しました');
        }
    };

    const startEditingMemo = (id) => {
        const memo = memos.find(m => m.id === id);
        setEditingMemoId(id);
        setEditMemoActiveTabs(prev => ({ ...prev, [id]: 'edit' }));
        setEditingMemoTexts(prev => ({ ...prev, [id]: memo?.text || '' }));
    };

    const cancelEditingMemo = async (id) => {
        const currentText = editingMemoTexts[id] || '';
        
        // メモが空の場合は削除
        if (!currentText.trim()) {
            await handleDeleteMemo(id);
            return;
        }
        
        // 編集を終了（保存せず）
        setEditingMemoId(null);
        setEditMemoActiveTabs(prev => {
            const newTabs = { ...prev };
            delete newTabs[id];
            return newTabs;
        });
        setEditingMemoTexts(prev => {
            const newTexts = { ...prev };
            delete newTexts[id];
            return newTexts;
        });
    };

    const handleAskToAgent = async (memoId) => {
        const query = askAgentQuery[memoId];
        if (!query || !query.trim()) {
            alert('質問内容を入力してください。');
            return;
        }

        setIsAskingAgent(prev => ({ ...prev, [memoId]: true }));
        try {
            const response = await paperNotesApi.askToAgent(memoId, query);
            
            // 元のメモを回答で置き換える
            setMemos(prev => prev.map(memo => 
                memo.id === memoId 
                    ? {
                        id: response.paper_note_id,
                        text: response.text,
                        createdAt: response.note_timestamp,
                        updatedAt: response.note_timestamp,
                        type: 'agent-response'
                    }
                    : memo
            ));
            
            setAskAgentQuery(prev => ({ ...prev, [memoId]: '' }));
            setShowAskAgentForm(prev => ({ ...prev, [memoId]: false }));
            
        } catch (err) {
            console.error('エージェントへの質問エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || 'エージェントへの質問に失敗しました');
        } finally {
            setIsAskingAgent(prev => ({ ...prev, [memoId]: false }));
        }
    };

    // その他のヘルパー関数
    const handleGenerateLlmMemo = async () => {
        if (process.env.NODE_ENV !== 'development') return;
        if (!newMemoText.trim()) return;
        
        setIsGeneratingLlmMemo(true);
        try {
            const enhancedMemo = `${newMemoText}

--- AIによる追加情報（ダミー） ---
この論文は自然言語処理の分野で革新的なTransformerアーキテクチャを提案しており、従来のRNNベースのモデルを大幅に上回る性能を示しています。特に注目すべき点は、並列処理が可能になったことで訓練時間が大幅に短縮された点です。

ユーザーのメモ内容に関連して、さらに詳しく分析すると、この研究の意義は機械翻訳だけでなく、後の多くのNLPタスクの基盤となったことです。`;
            setNewMemoText(enhancedMemo);
        } catch (err) {
            console.error('LLMメモ生成エラー:', err);
        } finally {
            setIsGeneratingLlmMemo(false);
        }
    };

    const handleGenerateEditMemo = async (memoId) => {
        if (process.env.NODE_ENV !== 'development') return;
        
        const currentText = editingMemoTexts[memoId] !== undefined ? editingMemoTexts[memoId] : memos.find(m => m.id === memoId)?.text || '';
        if (!currentText || !currentText.trim()) return;
        
        setIsGeneratingEditMemo(prev => ({ ...prev, [memoId]: true }));
        try {
            const enhancedText = `${currentText}

--- AIによる追加情報（ダミー） ---
この論文は自然言語処理の分野で革新的なTransformerアーキテクチャを提案しており、従来のRNNベースのモデルを大幅に上回る性能を示しています。特に注目すべき点は、並列処理が可能になったことで訓練時間が大幅に短縮された点です。

ユーザーのメモ内容に関連して、さらに詳しく分析すると、この研究の意義は機械翻訳だけでなく、後の多くのNLPタスクの基盤となったことです。`;
            setEditingMemoTexts(prev => ({ ...prev, [memoId]: enhancedText }));
        } catch (err) {
            console.error('LLMメモ生成エラー:', err);
        } finally {
            setIsGeneratingEditMemo(prev => ({ ...prev, [memoId]: false }));
        }
    };

    return {
        // States
        isAddingMemo,
        editingMemoId,
        newMemoText,
        isGeneratingLlmMemo,
        askAgentQuery,
        isAskingAgent,
        showAskAgentForm,
        addMemoActiveTab,
        editMemoActiveTabs,
        viewMemoActiveTabs,
        isGeneratingEditMemo,
        editingMemoTexts,
        
        // Setters
        setIsAddingMemo,
        setNewMemoText,
        setAddMemoActiveTab,
        setEditMemoActiveTabs,
        setViewMemoActiveTabs,
        setEditingMemoTexts,
        setAskAgentQuery,
        setShowAskAgentForm,
        
        // Actions
        handleAddMemo,
        handleEditMemo,
        handleDeleteMemo,
        startEditingMemo,
        cancelEditingMemo,
        handleAskToAgent,
        handleGenerateLlmMemo,
        handleGenerateEditMemo
    };
};
