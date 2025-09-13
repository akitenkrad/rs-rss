import 'highlight.js/styles/github-dark.css';
import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { mockPaperDetail } from '../../sample_data';
import { academicPapersApi, handleApiError, paperNotesApi } from '../api/Api';
import MermaidRenderer from '../MermaidRenderer';
import './AcademicPaperDetail.css';
import PaperContent from './components/PaperContent';
import PaperMemoSection from './components/PaperMemoSection';
import PaperMetaData from './components/PaperMetaData';
import { useMemoActions } from './hooks/useMemoActions';

const AcademicPaperDetail = () => {
    const { paper_id } = useParams();
    const navigate = useNavigate();
    const [paper, setPaper] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [isScrolled, setIsScrolled] = useState(false);
    const [memos, setMemos] = useState([]);
    
    // 本文の折りたたみ状態
    const [isFullTextExpanded, setIsFullTextExpanded] = useState(false);

    // メモ関連のカスタムフック
    const memoActions = useMemoActions(paper_id, memos, setMemos, setError);

    // Mermaidを含むコードブロックのカスタムレンダラー
    const CodeBlock = ({ node, inline, className, children, ...props }) => {
        const match = /language-(\w+)/.exec(className || '');
        const language = match ? match[1] : '';
        
        if (inline) {
            return <code className={className} {...props}>{children}</code>;
        }
        
        if (language === 'mermaid') {
            const chartContent = String(children).replace(/\n$/, '');
            return (
                <MermaidRenderer 
                    chart={chartContent} 
                    id={`chart-${Math.random().toString(36).substr(2, 9)}`}
                />
            );
        }
        
        return (
            <pre className={className} {...props}>
                <code>{children}</code>
            </pre>
        );
    };

    useEffect(() => {
        fetchPaperDetail(paper_id);
        fetchMemos(paper_id);
    }, [paper_id]);

    useEffect(() => {
        const handleScroll = () => {
            const scrollTop = window.scrollY;
            const headerHeight = 64;
            setIsScrolled(scrollTop > headerHeight);
        };

        window.addEventListener('scroll', handleScroll);
        return () => window.removeEventListener('scroll', handleScroll);
    }, []);

    const fetchPaperDetail = async (paperId) => {
        try {
            setLoading(true);
            setError(null);
            
            if (process.env.NODE_ENV === 'development') {
                await new Promise(resolve => setTimeout(resolve, 1000));
                setPaper(mockPaperDetail);
            } else {
                const paperData = await academicPapersApi.getById(paperId);
                setPaper(paperData);
            }
        } catch (err) {
            console.error('論文詳細取得エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || '論文の詳細情報の取得に失敗しました');
        } finally {
            setLoading(false);
        }
    };

    const fetchMemos = async (paperId) => {
        try {
            if (process.env.NODE_ENV === 'development') {
                setMemos([]);
            } else {
                const response = await paperNotesApi.getByPaperId(paperId);
                const fetchedMemos = response.paper_notes.map(note => ({
                    id: note.paper_note_id,
                    text: note.text,
                    type: 'manual',
                    createdAt: note.note_timestamp,
                    updatedAt: note.note_timestamp
                }));
                setMemos(fetchedMemos);
            }
        } catch (err) {
            console.error('メモ取得エラー:', err);
            setMemos([]);
        }
    };

    const formatDate = (dateString) => {
        if (!dateString) return '-';
        return new Date(dateString).toLocaleDateString('ja-JP', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    };

    const formatNumber = (number) => {
        if (number === null || number === undefined) return '-';
        return number.toLocaleString('ja-JP');
    };

    const handleCopyBibtex = async () => {
        try {
            await navigator.clipboard.writeText(paper.bibtex);
            console.log('Bibtex copied to clipboard');
        } catch (err) {
            console.error('Failed to copy bibtex: ', err);
            const textArea = document.createElement('textarea');
            textArea.value = paper.bibtex;
            document.body.appendChild(textArea);
            textArea.select();
            try {
                document.execCommand('copy');
                console.log('Bibtex copied to clipboard (fallback)');
            } catch (fallbackErr) {
                console.error('Fallback copy failed: ', fallbackErr);
            }
            document.body.removeChild(textArea);
        }
    };

    const handleBackToList = () => {
        navigate('/papers');
    };

    const handleUpdatePaper = async () => {
        try {
            setLoading(true);
            setError(null);
            
            if (process.env.NODE_ENV === 'development') {
                await new Promise(resolve => setTimeout(resolve, 1000));
                await fetchPaperDetail(paper_id);
            } else {
                const updatedPaper = await academicPapersApi.update(paper_id);
                setPaper(updatedPaper);
            }
        } catch (err) {
            console.error('論文更新エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || '論文の更新に失敗しました');
        } finally {
            setLoading(false);
        }
    };

    const handleKeywordClick = (keyword) => {
        navigate(`/papers?search=${encodeURIComponent(keyword)}`);
    };

    // メモ関連のヘルパー関数
    const handleTabChange = (memoId, tabType, tab) => {
        if (tabType === 'edit') {
            memoActions.setEditMemoActiveTabs(prev => ({ ...prev, [memoId]: tab }));
        } else {
            memoActions.setViewMemoActiveTabs(prev => ({ ...prev, [memoId]: tab }));
        }
    };

    const handleEditingTextChange = (memoId, text) => {
        memoActions.setEditingMemoTexts(prev => ({ ...prev, [memoId]: text }));
    };

    const handleCancelEditMemo = (memoId) => {
        memoActions.cancelEditingMemo(memoId);
    };

    const handleAskAgentQueryChange = (memoId, value) => {
        memoActions.setAskAgentQuery(prev => ({ ...prev, [memoId]: value }));
    };

    const handleShowAskAgentForm = (memoId) => {
        memoActions.setShowAskAgentForm(prev => ({ ...prev, [memoId]: true }));
    };

    const handleCancelAskAgent = (memoId) => {
        memoActions.setShowAskAgentForm(prev => ({ ...prev, [memoId]: false }));
        memoActions.setAskAgentQuery(prev => ({ ...prev, [memoId]: '' }));
    };

    const handleStartAddMemo = () => {
        memoActions.setIsAddingMemo(true);
        memoActions.setNewMemoText('');
    };

    const handleCancelAddMemo = () => {
        memoActions.setIsAddingMemo(false);
        memoActions.setNewMemoText('');
        memoActions.setAddMemoActiveTab('edit');
    };

    if (loading) {
        return (
            <div className="paper-detail">
                <div className="loading">論文の詳細情報を読み込み中...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="paper-detail">
                <div className="error">エラー: {error}</div>
                <button onClick={() => fetchPaperDetail(paper_id)} className="retry-button">
                    再試行
                </button>
            </div>
        );
    }

    if (!paper) {
        return (
            <div className="paper-detail">
                <div className="error">論文が見つかりませんでした</div>
            </div>
        );
    }

    return (
        <div className="paper-detail">
            <div className={`navigation-bar ${isScrolled ? 'scrolled' : ''}`}>
                <button onClick={handleBackToList} className="back-button">
                    ← 論文一覧に戻る
                </button>
            </div>

            <div className="paper-header">
                <h1 className="paper-title">{paper.title}</h1>
                <button 
                    onClick={handleUpdatePaper} 
                    className="update-button"
                    disabled={loading}
                >
                    {loading ? 'UPDATING...' : 'UPDATE PAPER'}
                </button>
            </div>

            <div className="paper-content">
                <PaperMetaData 
                    paper={paper}
                    onCopyBibtex={handleCopyBibtex}
                    onKeywordClick={handleKeywordClick}
                    formatDate={formatDate}
                    formatNumber={formatNumber}
                />

                <PaperContent 
                    paper={paper}
                    isFullTextExpanded={isFullTextExpanded}
                    onToggleFullText={() => setIsFullTextExpanded(!isFullTextExpanded)}
                    CodeBlock={CodeBlock}
                />

                <PaperMemoSection
                    memos={memos}
                    isAddingMemo={memoActions.isAddingMemo}
                    newMemoText={memoActions.newMemoText}
                    addMemoActiveTab={memoActions.addMemoActiveTab}
                    editingMemoId={memoActions.editingMemoId}
                    editingMemoTexts={memoActions.editingMemoTexts}
                    editMemoActiveTabs={memoActions.editMemoActiveTabs}
                    viewMemoActiveTabs={memoActions.viewMemoActiveTabs}
                    isGeneratingLlmMemo={memoActions.isGeneratingLlmMemo}
                    isGeneratingEditMemo={memoActions.isGeneratingEditMemo}
                    askAgentQuery={memoActions.askAgentQuery}
                    isAskingAgent={memoActions.isAskingAgent}
                    showAskAgentForm={memoActions.showAskAgentForm}
                    onStartAddMemo={handleStartAddMemo}
                    onAddMemo={memoActions.handleAddMemo}
                    onCancelAddMemo={handleCancelAddMemo}
                    onNewMemoTextChange={memoActions.setNewMemoText}
                    onAddMemoTabChange={memoActions.setAddMemoActiveTab}
                    onGenerateLlmMemo={memoActions.handleGenerateLlmMemo}
                    onStartEditMemo={memoActions.startEditingMemo}
                    onEditMemo={memoActions.handleEditMemo}
                    onCancelEditMemo={handleCancelEditMemo}
                    onDeleteMemo={memoActions.handleDeleteMemo}
                    onEditingTextChange={handleEditingTextChange}
                    onTabChange={handleTabChange}
                    onGenerateEditMemo={memoActions.handleGenerateEditMemo}
                    onAskAgentQueryChange={handleAskAgentQueryChange}
                    onAskToAgent={memoActions.handleAskToAgent}
                    onShowAskAgentForm={handleShowAskAgentForm}
                    onCancelAskAgent={handleCancelAskAgent}
                    CodeBlock={CodeBlock}
                />
            </div>
        </div>
    );
};

export default AcademicPaperDetail;