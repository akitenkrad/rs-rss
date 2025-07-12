import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import './AcademicPaperDetail.css';

// 開発用ダミーデータ
const mockPaperDetail = {
    id: 1,
    title: 'Attention Is All You Need',
    authors: [
        {
            name: 'Ashish Vaswani',
            h_index: 45,
            link: 'https://scholar.google.com/citations?user=example1'
        },
        {
            name: 'Noam Shazeer',
            h_index: 38,
            link: 'https://scholar.google.com/citations?user=example2'
        },
        {
            name: 'Niki Parmar',
            h_index: 25,
            link: 'https://scholar.google.com/citations?user=example3'
        },
        {
            name: 'Jakob Uszkoreit',
            h_index: 28,
            link: 'https://scholar.google.com/citations?user=example4'
        }
    ],
    abstract_text: 'The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely. Experiments on two machine translation tasks show these models to be superior in quality while being more parallelizable and requiring significantly less time to train.',
    url: 'https://arxiv.org/abs/1706.03762',
    published_date: '2017-06-12',
    journal: 'Advances in Neural Information Processing Systems',
    keywords: ['Attention Mechanism', 'Transformer', 'Neural Machine Translation', 'Deep Learning', 'Natural Language Processing'],
    background_and_purpose: 'Recurrent neural networks, long short-term memory and gated recurrent neural networks in particular, have been firmly established as state of the art approaches in sequence modeling and transduction problems such as language modeling and machine translation. Numerous efforts have since continued to push the boundaries of recurrent language models and encoder-decoder architectures.',
    methodology: 'The goal of reducing sequential computation also forms the foundation of the Extended Neural GPU, ByteNet and ConvS2S, all of which use convolutional neural networks as basic building block, computing hidden representations in parallel for all input and output positions. In these models, the number of operations required to relate signals from two arbitrary input or output positions grows in the distance between positions, linearly for ConvS2S and logarithmically for ByteNet.',
    dataset: 'We trained on the standard WMT 2014 English-German dataset consisting of about 4.5 million sentence pairs. We also used the larger WMT 2014 English-French dataset consisting of 36M sentences and split tokens into a 32000 word-piece vocabulary.',
    results: 'We evaluate our models on two machine translation tasks: WMT 2014 English-to-German and WMT 2014 English-to-French. For the smaller English-German dataset, we achieved a BLEU score of 28.4, which is competitive with the best previously reported results. For the larger English-French dataset, we achieved a BLEU score of 41.8, establishing a new state-of-the-art.',
    future_works: 'We plan to extend the Transformer to problems involving input and output modalities other than text, such as images, audio and video. Making generation less sequential is another research goals of ours. We also plan to investigate local, restricted attention mechanisms to efficiently handle very long sequences.'
};

const AcademicPaperDetail = () => {
    const { paper_id } = useParams();
    const navigate = useNavigate();
    const [paper, setPaper] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [isScrolled, setIsScrolled] = useState(false);

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
                const response = await fetch(`http://localhost:8080/api/v1/academic_paper/paper?paper_id=${paperId}`);
                if (!response.ok) {
                    throw new Error('論文の詳細情報の取得に失敗しました');
                }
                const data = await response.json();
                setPaper(data);
            }
        } catch (err) {
            setError(err.message);
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

    const handleBackToList = () => {
        navigate('/papers');
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
                <div className="paper-meta">
                    <span className="published-date">{formatDate(paper.published_date)}</span>
                    <span className="journal">{paper.journal.name}</span>
                </div>
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
                            {paper.keywords || [].map((keyword, index) => ( // TODO: Replace with actual keywords
                                <span key={index} className="keyword-tag">
                                    {keyword}
                                </span>
                            ))}
                        </div>
                    </div>

                    <div className="links-section">
                        <h2>Links</h2>
                        <div className="links-divider"></div>
                        <div className="links-container">
                            <a 
                                href={paper.url} 
                                target="_blank" 
                                rel="noopener noreferrer"
                                className="paper-link-button"
                            >
                                Read Paper
                            </a>
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
            </div>
        </div>
    );
};

export default AcademicPaperDetail;