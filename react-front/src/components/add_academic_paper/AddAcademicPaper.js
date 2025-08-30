import { useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { academicPapersApi, handleApiError } from '../api/Api';
import './AddAcademicPaper.css';

const AddAcademicPaper = () => {
    const navigate = useNavigate();
    const sseCleanupRef = useRef(null);
    const [formData, setFormData] = useState({
        title: '',
        pdfUrl: ''
    });
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState(null);
    const [success, setSuccess] = useState(false);
    const [progress, setProgress] = useState(0);
    const [progressMessage, setProgressMessage] = useState('');
    const [completedPaper, setCompletedPaper] = useState(null);

    const handleInputChange = (e) => {
        const { name, value } = e.target;
        setFormData(prev => ({
            ...prev,
            [name]: value
        }));
        // エラーをクリア
        if (error) {
            setError(null);
        }
    };

    const validateForm = () => {
        if (!formData.title.trim()) {
            setError('論文のタイトルを入力してください。');
            return false;
        }
        if (!formData.pdfUrl.trim()) {
            setError('PDFのURLを入力してください。');
            return false;
        }
        
        // URLの簡単な検証
        try {
            new URL(formData.pdfUrl);
        } catch (e) {
            setError('有効なURLを入力してください。');
            return false;
        }

        return true;
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        
        if (!validateForm()) {
            return;
        }

        setIsSubmitting(true);
        setError(null);
        setProgress(0);
        setProgressMessage('');
        setSuccess(false);

        try {
            const cleanup = await academicPapersApi.createWithSSE(
                {
                    title: formData.title.trim(),
                    pdf_url: formData.pdfUrl.trim()
                },
                // onProgress callback
                (data) => {
                    console.log('Progress data received:', data);
                    console.log('Data type:', typeof data);
                    console.log('Data keys:', Object.keys(data || {}));
                    
                    try {
                        if (data.progress !== undefined) {
                            setProgress(data.progress);
                        }
                        if (data.message) {
                            // Ensure message is a string
                            const message = typeof data.message === 'string' ? data.message : JSON.stringify(data.message);
                            setProgressMessage(message);
                        }
                        
                        // If paper data is included in progress updates, ensure it doesn't cause rendering issues
                        if (data.paper) {
                            console.log('Paper data in progress:', data.paper);
                            console.log('Paper authors type:', typeof data.paper.authors);
                            console.log('Paper authors:', data.paper.authors);
                        }
                    } catch (error) {
                        console.error('Error processing progress data:', error);
                        setProgressMessage('Processing...');
                    }
                },
                // onError callback
                (error) => {
                    console.error('SSE論文追加エラー:', error);
                    setError('論文の追加中にエラーが発生しました。');
                    setIsSubmitting(false);
                },
                // onComplete callback
                (data) => {
                    console.log('Received paper data:', data);
                    console.log('Paper authors:', data.paper?.authors);
                    setSuccess(true);
                    setCompletedPaper(data.paper);
                    setProgress(100);
                    setProgressMessage('論文の追加が完了しました！');
                    
                    // 成功後は3秒待ってから論文詳細ページに遷移
                    setTimeout(() => {
                        if (data.paper && data.paper.id) {
                            navigate(`/papers/${data.paper.id}`);
                        } else {
                            navigate('/papers');
                        }
                    }, 3000);
                    setIsSubmitting(false);
                }
            );
            
            // Store cleanup function for potential cancellation
            sseCleanupRef.current = cleanup;
            
        } catch (err) {
            console.error('論文追加エラー:', err);
            const apiError = handleApiError(err);
            setError(apiError.message || '論文の追加に失敗しました。');
            setIsSubmitting(false);
        }
    };

    const handleReset = () => {
        // Cancel any ongoing SSE connection
        if (sseCleanupRef.current) {
            sseCleanupRef.current();
            sseCleanupRef.current = null;
        }
        
        setFormData({
            title: '',
            pdfUrl: ''
        });
        setError(null);
        setSuccess(false);
        setProgress(0);
        setProgressMessage('');
        setCompletedPaper(null);
        setIsSubmitting(false);
    };

    if (success) {
        return (
            <div className="add-paper-container">
                <div className="success-message">
                    <h2>論文が正常に追加されました！</h2>
                    {completedPaper && (
                        <div className="paper-info">
                            <h3>追加された論文:</h3>
                            <p><strong>タイトル:</strong> {completedPaper.title}</p>
                            {completedPaper.authors && (
                                <p><strong>著者:</strong> {
                                    (() => {
                                        try {
                                            if (Array.isArray(completedPaper.authors)) {
                                                return completedPaper.authors.map((author, index) => {
                                                    if (typeof author === 'string') {
                                                        return author;
                                                    } else if (author && typeof author === 'object' && author.name) {
                                                        return author.name;
                                                    } else {
                                                        return `Author ${index + 1}`;
                                                    }
                                                }).join(', ');
                                            } else if (typeof completedPaper.authors === 'string') {
                                                return completedPaper.authors;
                                            } else {
                                                return 'Unknown authors';
                                            }
                                        } catch (error) {
                                            console.error('Error rendering authors:', error);
                                            return 'Error displaying authors';
                                        }
                                    })()
                                }</p>
                            )}
                        </div>
                    )}
                    <p>論文詳細ページに移動しています...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="add-paper-container">
            <div className="add-paper-header">
                <h1>Add a New Paper</h1>
            </div>

            <form onSubmit={handleSubmit} className="add-paper-form">
                <div className="form-group">
                    <label htmlFor="title" className="form-label">
                        論文タイトル <span className="required">*</span>
                    </label>
                    <input
                        type="text"
                        id="title"
                        name="title"
                        value={formData.title}
                        onChange={handleInputChange}
                        className="form-input"
                        placeholder="例: Attention Is All You Need"
                        disabled={isSubmitting}
                    />
                </div>

                <div className="form-group">
                    <label htmlFor="pdfUrl" className="form-label">
                        PDF URL <span className="required">*</span>
                    </label>
                    <input
                        type="url"
                        id="pdfUrl"
                        name="pdfUrl"
                        value={formData.pdfUrl}
                        onChange={handleInputChange}
                        className="form-input"
                        placeholder="例: https://arxiv.org/pdf/1706.03762.pdf"
                        disabled={isSubmitting}
                    />
                </div>

                {error && (
                    <div className="error-message">
                        {error}
                    </div>
                )}

                {/* Progress Indicator */}
                {isSubmitting && (
                    <div className="progress-container">
                        <div className="progress-bar">
                            <div 
                                className="progress-fill" 
                                style={{ width: `${progress}%` }}
                            ></div>
                        </div>
                        <div className="progress-info">
                            <span className="progress-percentage">{progress}%</span>
                            {progressMessage && (
                                <span className="progress-message">
                                    {typeof progressMessage === 'string' ? progressMessage : 'Processing...'}
                                </span>
                            )}
                        </div>
                    </div>
                )}

                <div className="form-actions">
                    <button
                        type="button"
                        onClick={handleReset}
                        className="reset-btn"
                        disabled={isSubmitting}
                    >
                        {isSubmitting ? 'キャンセル' : 'リセット'}
                    </button>
                    <button
                        type="submit"
                        className="submit-btn"
                        disabled={isSubmitting || !formData.title.trim() || !formData.pdfUrl.trim()}
                    >
                        {isSubmitting ? (
                            <>
                                <span className="loading-spinner"></span>
                                {typeof progressMessage === 'string' && progressMessage ? progressMessage : '処理中...'}
                            </>
                        ) : (
                            '論文を追加'
                        )}
                    </button>
                </div>
            </form>
        </div>
    );
};

export default AddAcademicPaper;
