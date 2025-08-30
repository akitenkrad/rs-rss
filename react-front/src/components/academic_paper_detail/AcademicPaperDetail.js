import 'highlight.js/styles/github-dark.css';
import { useEffect, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import { useNavigate, useParams } from 'react-router-dom';
import rehypeHighlight from 'rehype-highlight';
import rehypeRaw from 'rehype-raw';
import remarkGfm from 'remark-gfm';
import { mockPaperDetail } from '../../sample_data';
import { academicPapersApi, handleApiError, llmApi } from '../api/Api';
import MermaidRenderer from '../MermaidRenderer';
import './AcademicPaperDetail.css';

const AcademicPaperDetail = () => {
    const { paper_id } = useParams();
    const navigate = useNavigate();
    const [paper, setPaper] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [isScrolled, setIsScrolled] = useState(false);
    
    // 本文の折りたたみ状態
    const [isFullTextExpanded, setIsFullTextExpanded] = useState(false);
    
    // メモ関連のstate
    const [memos, setMemos] = useState([]);
    const [isAddingMemo, setIsAddingMemo] = useState(false);
    const [editingMemoId, setEditingMemoId] = useState(null);
    const [newMemoText, setNewMemoText] = useState('');
    const [isGeneratingLlmMemo, setIsGeneratingLlmMemo] = useState(false);
    
    // タブ管理用のstate
    const [addMemoActiveTab, setAddMemoActiveTab] = useState('edit');
    const [editMemoActiveTabs, setEditMemoActiveTabs] = useState({});
    const [viewMemoActiveTabs, setViewMemoActiveTabs] = useState({});
    const [isGeneratingEditMemo, setIsGeneratingEditMemo] = useState({});
    
    // 編集中のメモ内容を管理するstate
    const [editingMemoTexts, setEditingMemoTexts] = useState({});

    // Mermaidを含むコードブロックのカスタムレンダラー
    const CodeBlock = ({ node, inline, className, children, ...props }) => {
        const match = /language-(\w+)/.exec(className || '');
        const language = match ? match[1] : '';
        
        // インラインコードの場合は通常のcodeタグを返す
        if (inline) {
            return <code className={className} {...props}>{children}</code>;
        }
        
        // Mermaidダイアグラムの場合
        if (language === 'mermaid') {
            const chartContent = String(children).replace(/\n$/, '');
            return (
                <MermaidRenderer 
                    chart={chartContent} 
                    id={`chart-${Math.random().toString(36).substr(2, 9)}`}
                />
            );
        }
        
        // その他のコードブロック
        return (
            <pre className={className} {...props}>
                <code>{children}</code>
            </pre>
        );
    };

    useEffect(() => {
        fetchPaperDetail(paper_id);
    }, [paper_id]);

    useEffect(() => {
        const handleScroll = () => {
            const scrollTop = window.scrollY;
            const headerHeight = 64; // ヘッダーの高さ（調整が必要な場合があります）
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
            // 成功時の視覚的フィードバック（オプション）
            console.log('Bibtex copied to clipboard');
            // TODO: トースト通知やボタンの一時的な状態変更を追加可能
        } catch (err) {
            console.error('Failed to copy bibtex: ', err);
            // フォールバック: 古いブラウザ対応
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
                // 開発環境では単純にリロード
                await new Promise(resolve => setTimeout(resolve, 1000));
                await fetchPaperDetail(paper_id);
            } else {
                // 本番環境では実際のAPI呼び出し
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
        // キーワードで検索する機能（将来実装予定）
        navigate(`/papers?search=${encodeURIComponent(keyword)}`);
    };

    // メモ関連の関数
    const handleAddMemo = () => {
        if (newMemoText.trim()) {
            const isLlmGenerated = newMemoText.includes('--- AIによる追加情報');
            const newMemo = {
                id: Date.now(),
                text: newMemoText.trim(),
                type: isLlmGenerated ? 'llm' : 'manual',
                createdAt: new Date().toISOString(),
                updatedAt: new Date().toISOString()
            };
            setMemos([...memos, newMemo]);
            setNewMemoText('');
            setIsAddingMemo(false);
            setAddMemoActiveTab('edit');
            // 新しく作成されたメモのプレビュータブをアクティブにする
            setViewMemoActiveTabs(prev => ({ ...prev, [newMemo.id]: 'preview' }));
        }
    };

    const handleEditMemo = (id, newText) => {
        setMemos(memos.map(memo => 
            memo.id === id 
                ? { ...memo, text: newText, updatedAt: new Date().toISOString() }
                : memo
        ));
        setEditingMemoId(null);
        // 編集完了後はプレビュータブをアクティブにする
        setViewMemoActiveTabs(prev => ({ ...prev, [id]: 'preview' }));
        // 編集中のテキストをクリア
        setEditingMemoTexts(prev => {
            const newTexts = { ...prev };
            delete newTexts[id];
            return newTexts;
        });
    };

    const handleDeleteMemo = (id) => {
        setMemos(memos.filter(memo => memo.id !== id));
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
    };

    const startEditingMemo = (id) => {
        const memo = memos.find(m => m.id === id);
        setEditingMemoId(id);
        // 編集開始時は編集タブをアクティブにする
        setEditMemoActiveTabs(prev => ({ ...prev, [id]: 'edit' }));
        // 編集中のテキストを初期化
        setEditingMemoTexts(prev => ({ ...prev, [id]: memo?.text || '' }));
    };

    const handleGenerateEditMemo = async (memoId) => {
        const currentText = editingMemoTexts[memoId] !== undefined ? editingMemoTexts[memoId] : memos.find(m => m.id === memoId)?.text || '';
        
        if (!currentText || !currentText.trim()) return;
        
        setIsGeneratingEditMemo(prev => ({ ...prev, [memoId]: true }));
        try {
            const result = await llmApi.generateMemo(paper, currentText);
            const enhancedText = `${currentText}

--- AIによる追加情報 ---
${result.memo}`;
            setEditingMemoTexts(prev => ({ ...prev, [memoId]: enhancedText }));
        } catch (err) {
            console.error('LLMメモ生成エラー:', err);
            // エラー時はダミーデータを追加（開発用）
            const enhancedText = `${currentText}

--- AIによる追加情報（ダミー） ---
この論文は自然言語処理の分野で革新的なTransformerアーキテクチャを提案しており、従来のRNNベースのモデルを大幅に上回る性能を示しています。特に注目すべき点は、並列処理が可能になったことで訓練時間が大幅に短縮された点です。

ユーザーのメモ内容に関連して、さらに詳しく分析すると、この研究の意義は機械翻訳だけでなく、後の多くのNLPタスクの基盤となったことです。`;
            setEditingMemoTexts(prev => ({ ...prev, [memoId]: enhancedText }));
        } finally {
            setIsGeneratingEditMemo(prev => ({ ...prev, [memoId]: false }));
        }
    };

    const handleGenerateLlmMemo = async () => {
        if (!newMemoText.trim()) return;
        
        setIsGeneratingLlmMemo(true);
        try {
            const result = await llmApi.generateMemo(paper, newMemoText);
            setNewMemoText(result.memo);
        } catch (err) {
            console.error('LLMメモ生成エラー:', err);
            // エラー時はダミーデータを追加（開発用）
            const enhancedMemo = `${newMemoText}

--- AIによる追加情報（ダミー） ---
この論文は自然言語処理の分野で革新的なTransformerアーキテクチャを提案しており、従来のRNNベースのモデルを大幅に上回る性能を示しています。特に注目すべき点は、並列処理が可能になったことで訓練時間が大幅に短縮された点です。

ユーザーのメモ内容に関連して、さらに詳しく分析すると、この研究の意義は機械翻訳だけでなく、後の多くのNLPタスクの基盤となったことです。`;
            setNewMemoText(enhancedMemo);
        } finally {
            setIsGeneratingLlmMemo(false);
        }
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
                <section className="basic-info">
                    <div className="authors-section">
                        <h2>Authors</h2>
                        <div className="authors-divider"></div>
                        <div className="authors-grid">
                            {paper.authors.map((author, index) => (
                                <div key={index} className="author-card">
                                    <a 
                                        href={author.link || '#'} //TODO: Replace with actual author link
                                        target="_blank" 
                                        rel="noopener noreferrer"
                                        className="author-link"
                                    >
                                        <span className="author-name">{author.name}</span>
                                        <span className="author-h-index">H-Index: {author.h_index}</span>
                                    </a>
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className="abstract-section">
                        <h2>Abstract</h2>
                        <div className="abstract-divider"></div>
                        <p className="abstract-text">{paper.abstract_text}</p>
                    </div>

                    <div className="keywords-section">
                        <h2>Keywords</h2>
                        <div className="keywords-divider"></div>
                        <div className="keywords-container">
                            {(paper.keywords || []).map((keyword, index) => (
                                <span 
                                    key={index} 
                                    className="keyword-tag"
                                    onClick={() => handleKeywordClick(keyword)}
                                    tabIndex={0}
                                    onKeyDown={(e) => {
                                        if (e.key === 'Enter' || e.key === ' ') {
                                            e.preventDefault();
                                            handleKeywordClick(keyword);
                                        }
                                    }}
                                    role="button"
                                    aria-label={`Search for papers with keyword: ${keyword}`}
                                >
                                    <span className="keyword-icon">#</span>
                                    {keyword}
                                </span>
                            ))}
                        </div>
                    </div>

                    <div className="meta-data-section">
                        <h2>Meta Data</h2>
                        <div className="meta-data-divider"></div>
                        
                        <div className="meta-data-content">
                            <div className="meta-info-grid">
                                <div className="meta-info-item">
                                    <span className="meta-label">Published Date</span>
                                    <span className="meta-value published-date">{formatDate(paper.published_date)}</span>
                                </div>
                                <div className="meta-info-item">
                                    <span className="meta-label">Journal</span>
                                    <span className="meta-value journal">{paper.journal.name}</span>
                                </div>
                                <div className="meta-info-item">
                                    <span className="meta-label">Primary Category</span>
                                    <span className="meta-value primary-category">{paper.primary_category || 'N/A'}</span>
                                </div>
                                <div className="meta-info-item">
                                    <span className="meta-label">Paper Link</span>
                                    <a 
                                        href={paper.url} 
                                        target="_blank" 
                                        rel="noopener noreferrer"
                                        className="meta-value paper-link"
                                    >
                                        Read Paper
                                    </a>
                                </div>
                            </div>
                            
                            <div className="citation-metrics-grid">
                                <div className="metric-card">
                                    <span className="metric-label">Citation Count</span>
                                    <span className="metric-value">{formatNumber(paper.citation_count)}</span>
                                </div>
                                <div className="metric-card">
                                    <span className="metric-label">Reference Count</span>
                                    <span className="metric-value">{formatNumber(paper.reference_count)}</span>
                                </div>
                                <div className="metric-card">
                                    <span className="metric-label">Influential Citations</span>
                                    <span className="metric-value">{formatNumber(paper.influential_citation_count)}</span>
                                </div>
                            </div>
                            
                            <div className="bibtex-section">
                                <span className="meta-label">Bibtex</span>
                                <div className="bibtex-container">
                                    <pre className="bibtex-text">{paper.bibtex}</pre>
                                    <button 
                                        className="copy-bibtex-btn"
                                        onClick={handleCopyBibtex}
                                        title="Copy Bibtex to clipboard"
                                    >
                                        📋 Copy
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section className="detailed-content">
                    <h2>Contents</h2>
                    
                    <div className="content-section">
                        <h3>Background & Research Question</h3>
                        <div className="section-divider"></div>
                        <p className="content-text">{paper.background_and_purpose}</p>
                    </div>

                    <div className="content-section">
                        <h3>Methodology</h3>
                        <div className="section-divider"></div>
                        <p className="content-text">{paper.methodology}</p>
                    </div>

                    <div className="content-section">
                        <h3>Dataset</h3>
                        <div className="section-divider"></div>
                        <p className="content-text">{paper.dataset}</p>
                    </div>

                    <div className="content-section">
                        <h3>Experiment Overview and Results</h3>
                        <div className="section-divider"></div>
                        <p className="content-text">{paper.results}</p>
                    </div>

                    <div className="content-section">
                        <h3>Future Works</h3>
                        <div className="section-divider"></div>
                        <p className="content-text">{paper.advantages_limitations_and_future_work}</p>
                    </div>
                </section>

                <section className="full-text-section">
                    <div className="full-text-header">
                        <h2>Full Text</h2>
                        <button 
                            className={`expand-button ${isFullTextExpanded ? 'expanded' : ''}`}
                            onClick={() => setIsFullTextExpanded(!isFullTextExpanded)}
                        >
                            {isFullTextExpanded ? 'Collapse' : 'Expand'} Full Text
                            <span className={`expand-icon ${isFullTextExpanded ? 'rotated' : ''}`}>▼</span>
                        </button>
                    </div>
                    <div className="full-text-divider"></div>
                    
                    <div className={`full-text-content ${isFullTextExpanded ? 'expanded' : 'collapsed'}`}>
                        {paper.text && typeof paper.text === 'string' && paper.text.length > 0 ? (
                            <div className="text-sections">
                                <div className="text-section">
                                    <ReactMarkdown
                                        remarkPlugins={[remarkGfm]}
                                        rehypePlugins={[rehypeRaw, rehypeHighlight]}
                                        components={{
                                            code: CodeBlock
                                        }}
                                    >
                                        {paper.text}
                                    </ReactMarkdown>
                                </div>
                            </div>
                        ) : paper.text && Array.isArray(paper.text) && paper.text.length > 0 ? (
                            <div className="text-sections">
                                {paper.text.map((section, index) => (
                                    <div key={index} className="text-section">
                                        <ReactMarkdown
                                            remarkPlugins={[remarkGfm]}
                                            rehypePlugins={[rehypeRaw, rehypeHighlight]}
                                            components={{
                                                code: CodeBlock
                                            }}
                                        >
                                            {section}
                                        </ReactMarkdown>
                                    </div>
                                ))}
                            </div>
                        ) : (
                            <div className="no-text-message">
                                <p>Full text is not available for this paper.</p>
                            </div>
                        )}
                    </div>
                </section>

                <section className="memos-section">
                    <h2>Notes & Memos</h2>
                    
                    {/* メモ追加エリア */}
                    <div className="memo-add-section">
                        <div className="memo-add-buttons">
                            <button 
                                className="add-memo-btn"
                                onClick={() => {
                                    setIsAddingMemo(true);
                                    setNewMemoText('');
                                }}
                            >
                                ✏️ メモを追加
                            </button>
                        </div>

                        {/* メモ追加フォーム */}
                        {isAddingMemo && (
                            <div className="memo-form">
                                <div className="memo-tabs">
                                    <button 
                                        className={`memo-tab ${addMemoActiveTab === 'edit' ? 'active' : ''}`}
                                        onClick={() => setAddMemoActiveTab('edit')}
                                    >
                                        📝 編集
                                    </button>
                                    <button 
                                        className={`memo-tab ${addMemoActiveTab === 'preview' ? 'active' : ''}`}
                                        onClick={() => setAddMemoActiveTab('preview')}
                                    >
                                        👁️ プレビュー
                                    </button>
                                </div>
                                
                                <div className="memo-tab-content">
                                    {addMemoActiveTab === 'edit' ? (
                                        <textarea
                                            className="memo-textarea"
                                            placeholder="メモをマークダウンで入力してください。AIに問い合わせる場合は、質問や依頼内容を書いてから「AIに問い合わせ」ボタンを押してください。&#10;&#10;例:&#10;# 重要なポイント&#10;- **Attention機構**の革新性&#10;- `Self-Attention`により並列処理が可能&#10;&#10;## 質問&#10;この論文の限界は何か？"
                                            value={newMemoText}
                                            onChange={(e) => setNewMemoText(e.target.value)}
                                            rows={6}
                                        />
                                    ) : (
                                        <div className="memo-preview">
                                            {newMemoText.trim() ? (
                                                <ReactMarkdown
                                                    remarkPlugins={[remarkGfm]}
                                                    rehypePlugins={[rehypeHighlight, rehypeRaw]}
                                                    components={{
                                                        code: CodeBlock,
                                                        pre: ({ children }) => <>{children}</>
                                                    }}
                                                >
                                                    {newMemoText}
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
                                        onClick={handleAddMemo}
                                        disabled={!newMemoText.trim()}
                                    >
                                        保存
                                    </button>
                                    <button 
                                        className="cancel-memo-btn"
                                        onClick={() => {
                                            setIsAddingMemo(false);
                                            setNewMemoText('');
                                            setAddMemoActiveTab('edit');
                                        }}
                                    >
                                        キャンセル
                                    </button>
                                    <button 
                                        className="ask-ai-btn"
                                        onClick={handleGenerateLlmMemo}
                                        disabled={!newMemoText.trim() || isGeneratingLlmMemo}
                                    >
                                        {isGeneratingLlmMemo ? '🤖 AIに問い合わせ中...' : '🤖 AIに問い合わせ'}
                                    </button>
                                </div>
                            </div>
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
                                <div key={memo.id} className={`memo-item ${memo.type}`}>
                                    <div className="memo-header">
                                        <div className="memo-date">
                                            {new Date(memo.createdAt).toLocaleString('ja-JP')}
                                        </div>
                                        <div className="memo-actions">
                                            <button 
                                                className="edit-memo-btn"
                                                onClick={() => startEditingMemo(memo.id)}
                                            >
                                                編集
                                            </button>
                                            <button 
                                                className="delete-memo-btn"
                                                onClick={() => handleDeleteMemo(memo.id)}
                                            >
                                                削除
                                            </button>
                                        </div>
                                    </div>
                                    
                                    {editingMemoId === memo.id ? (
                                        <div className="memo-edit-form">
                                            <div className="memo-tabs">
                                                <button 
                                                    className={`memo-tab ${(editMemoActiveTabs[memo.id] || 'edit') === 'edit' ? 'active' : ''}`}
                                                    onClick={() => setEditMemoActiveTabs(prev => ({ ...prev, [memo.id]: 'edit' }))}
                                                >
                                                    📝 編集
                                                </button>
                                                <button 
                                                    className={`memo-tab ${editMemoActiveTabs[memo.id] === 'preview' ? 'active' : ''}`}
                                                    onClick={() => setEditMemoActiveTabs(prev => ({ ...prev, [memo.id]: 'preview' }))}
                                                >
                                                    👁️ プレビュー
                                                </button>
                                            </div>
                                            
                                            <div className="memo-tab-content">
                                                {(editMemoActiveTabs[memo.id] || 'edit') === 'edit' ? (
                                                    <textarea
                                                        className="memo-edit-textarea"
                                                        value={editingMemoTexts[memo.id] !== undefined ? editingMemoTexts[memo.id] : memo.text}
                                                        onChange={(e) => {
                                                            setEditingMemoTexts(prev => ({ 
                                                                ...prev, 
                                                                [memo.id]: e.target.value 
                                                            }));
                                                        }}
                                                        rows={6}
                                                        id={`edit-textarea-${memo.id}`}
                                                        onKeyDown={(e) => {
                                                            if (e.key === 'Enter' && e.ctrlKey) {
                                                                const currentText = editingMemoTexts[memo.id] !== undefined ? editingMemoTexts[memo.id] : memo.text;
                                                                handleEditMemo(memo.id, currentText);
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
                                                            {editingMemoTexts[memo.id] !== undefined ? editingMemoTexts[memo.id] : memo.text}
                                                        </ReactMarkdown>
                                                    </div>
                                                )}
                                            </div>
                                            
                                            <div className="memo-edit-buttons">
                                                <button 
                                                    className="save-edit-btn"
                                                    onClick={(e) => {
                                                        const currentText = editingMemoTexts[memo.id] !== undefined ? editingMemoTexts[memo.id] : memo.text;
                                                        handleEditMemo(memo.id, currentText);
                                                    }}
                                                >
                                                    保存
                                                </button>
                                                <button 
                                                    className="cancel-edit-btn"
                                                    onClick={() => {
                                                        setEditingMemoId(null);
                                                        setEditMemoActiveTabs(prev => {
                                                            const newTabs = { ...prev };
                                                            delete newTabs[memo.id];
                                                            return newTabs;
                                                        });
                                                        setEditingMemoTexts(prev => {
                                                            const newTexts = { ...prev };
                                                            delete newTexts[memo.id];
                                                            return newTexts;
                                                        });
                                                    }}
                                                >
                                                    キャンセル
                                                </button>
                                                <button 
                                                    className="ask-ai-btn"
                                                    onClick={() => handleGenerateEditMemo(memo.id)}
                                                    disabled={isGeneratingEditMemo[memo.id]}
                                                >
                                                    {isGeneratingEditMemo[memo.id] ? '🤖 AIに問い合わせ中...' : '🤖 AIに問い合わせ'}
                                                </button>
                                            </div>
                                        </div>
                                    ) : (
                                        <div className="memo-view">
                                            <div className="memo-tabs">
                                                <button 
                                                    className={`memo-tab ${(viewMemoActiveTabs[memo.id] || 'preview') === 'preview' ? 'active' : ''}`}
                                                    onClick={() => setViewMemoActiveTabs(prev => ({ ...prev, [memo.id]: 'preview' }))}
                                                >
                                                    👁️ プレビュー
                                                </button>
                                                <button 
                                                    className={`memo-tab ${viewMemoActiveTabs[memo.id] === 'source' ? 'active' : ''}`}
                                                    onClick={() => setViewMemoActiveTabs(prev => ({ ...prev, [memo.id]: 'source' }))}
                                                >
                                                    📄 ソース
                                                </button>
                                            </div>
                                            
                                            <div className="memo-tab-content">
                                                {(viewMemoActiveTabs[memo.id] || 'preview') === 'preview' ? (
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
                                        </div>
                                    )}
                                </div>
                            ))
                        )}
                    </div>
                </section>
            </div>
        </div>
    );
};

export default AcademicPaperDetail;