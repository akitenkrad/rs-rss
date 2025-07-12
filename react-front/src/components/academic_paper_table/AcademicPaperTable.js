import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './AcademicPaperTable.css';

// 開発用ダミーデータ
const mockPapers = [
    {
        paper_id: 1,
        title: 'Attention Is All You Need',
        authors: 'Ashish Vaswani, Noam Shazeer, Niki Parmar, Jakob Uszkoreit, Llion Jones, Aidan N. Gomez, Lukasz Kaiser, Illia Polosukhin',
        published_date: '2017-06-12',
        journal: 'Advances in Neural Information Processing Systems',
        abstract: 'The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.',
        url: 'https://arxiv.org/abs/1706.03762',
        keywords: ['Attention Mechanism', 'Transformer', 'Neural Machine Translation'],
        primary_category: 'cs.CL'
    },
    {
        paper_id: 2,
        title: 'BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding',
        authors: 'Jacob Devlin, Ming-Wei Chang, Kenton Lee, Kristina Toutanova',
        published_date: '2018-10-11',
        journal: 'North American Chapter of the Association for Computational Linguistics',
        abstract: 'We introduce a new language representation model called BERT, which stands for Bidirectional Encoder Representations from Transformers. Unlike recent language representation models, BERT is designed to pre-train deep bidirectional representations from unlabeled text by jointly conditioning on both left and right context in all layers.',
        url: 'https://arxiv.org/abs/1810.04805',
        keywords: ['BERT', 'Language Model', 'Natural Language Processing'],
        primary_category: 'cs.CL'
    },
    {
        paper_id: 3,
        title: 'GPT-3: Language Models are Few-Shot Learners',
        authors: 'Tom B. Brown, Benjamin Mann, Nick Ryder, Melanie Subbiah, Jared Kaplan, Prafulla Dhariwal, Arvind Neelakantan, Pranav Shyam, Girish Sastry, Amanda Askell, et al.',
        published_date: '2020-05-28',
        journal: 'Advances in Neural Information Processing Systems',
        abstract: 'Recent work has demonstrated substantial gains on many NLP tasks and benchmarks by pre-training on a large corpus of text followed by fine-tuning on a specific task. While typically task-agnostic in architecture, this method still requires task-specific fine-tuning datasets of thousands or tens of thousands of examples.',
        url: 'https://arxiv.org/abs/2005.14165',
        keywords: ['GPT-3', 'Language Model', 'Few-Shot Learning'],
        primary_category: 'cs.CL'
    },
    {
        paper_id: 4,
        title: 'ResNet: Deep Residual Learning for Image Recognition',
        authors: 'Kaiming He, Xiangyu Zhang, Shaoqing Ren, Jian Sun',
        published_date: '2015-12-10',
        journal: 'IEEE Conference on Computer Vision and Pattern Recognition',
        abstract: 'Deeper neural networks are more difficult to train. We present a residual learning framework to ease the training of networks that are substantially deeper than those used previously. We explicitly reformulate the layers as learning residual functions with reference to the layer inputs, instead of learning unreferenced functions.',
        url: 'https://arxiv.org/abs/1512.03385',
        keywords: ['ResNet', 'Deep Learning', 'Computer Vision'],
        primary_category: 'cs.CV'
    },
    {
        paper_id: 5,
        title: 'Generative Adversarial Networks',
        authors: 'Ian J. Goodfellow, Jean Pouget-Abadie, Mehdi Mirza, Bing Xu, David Warde-Farley, Sherjil Ozair, Aaron Courville, Yoshua Bengio',
        published_date: '2014-06-10',
        journal: 'Advances in Neural Information Processing Systems',
        abstract: 'We propose a new framework for estimating generative models via an adversarial process, in which we simultaneously train two models: a generative model G that captures the data distribution, and a discriminative model D that estimates the probability that a sample came from the training data rather than G.',
        url: 'https://arxiv.org/abs/1406.2661',
        keywords: ['GAN', 'Generative Model', 'Deep Learning'],
        primary_category: 'cs.LG'
    },
    {
        paper_id: 6,
        title: 'Adam: A Method for Stochastic Optimization',
        authors: 'Diederik P. Kingma, Jimmy Ba',
        published_date: '2014-12-22',
        journal: 'International Conference on Learning Representations',
        abstract: 'We introduce Adam, an algorithm for first-order gradient-based optimization of stochastic objective functions, based on adaptive estimates of lower-order moments. The method is straightforward to implement, is computationally efficient, has little memory requirements, is invariant to diagonal rescaling of the gradients, and is well suited for problems that are large in terms of data and/or parameters.',
        url: 'https://arxiv.org/abs/1412.6980',
        keywords: ['Adam', 'Optimization', 'Machine Learning'],
        primary_category: 'cs.LG'
    },
    {
        paper_id: 7,
        title: 'Dropout: A Simple Way to Prevent Neural Networks from Overfitting',
        authors: 'Nitish Srivastava, Geoffrey Hinton, Alex Krizhevsky, Ilya Sutskever, Ruslan Salakhutdinov',
        published_date: '2014-06-15',
        journal: 'Journal of Machine Learning Research',
        abstract: 'Deep neural nets with a large number of parameters are very powerful machine learning systems. However, overfitting is a serious problem in such networks. Large networks are also slow to use, making it difficult to deal with overfitting by combining the predictions of many different large neural nets at test time.',
        url: 'https://www.cs.toronto.edu/~rsalakhu/papers/srivastava14a.pdf',
        keywords: ['Dropout', 'Regularization', 'Neural Networks'],
        primary_category: 'cs.LG'
    },
    {
        paper_id: 8,
        title: 'You Only Look Once: Unified, Real-Time Object Detection',
        authors: 'Joseph Redmon, Santosh Divvala, Ross Girshick, Ali Farhadi',
        published_date: '2016-05-09',
        journal: 'IEEE Conference on Computer Vision and Pattern Recognition',
        abstract: 'We present YOLO, a new approach to object detection. Prior work on object detection repurposes classifiers to perform detection. Instead, we frame object detection as a regression problem to spatially separated bounding boxes and associated class probabilities.',
        url: 'https://arxiv.org/abs/1506.02640',
        keywords: ['YOLO', 'Object Detection', 'Real-time'],
        primary_category: 'cs.CV'
    }
];

const AcademicPaperTable = () => {
    const navigate = useNavigate();
    const [papers, setPapers] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
    const [searchKeyword, setSearchKeyword] = useState('');
    const [isSearching, setIsSearching] = useState(false);

    useEffect(() => {
        fetchPapers();
    }, []);

    // 検索キーワードが変更されたときの処理（デバウンス付き）
    useEffect(() => {
        const debounceTimer = setTimeout(() => {
            if (searchKeyword !== '') {
                fetchPapers(searchKeyword);
            } else {
                fetchPapers();
            }
        }, 500); // 500ms の遅延でAPIを呼び出し

        return () => clearTimeout(debounceTimer);
    }, [searchKeyword]);

    const fetchPapers = async (keyword = '') => {
        try {
            setLoading(true);
            setError(null);
            
            // 開発環境の場合はダミーデータを使用
            if (process.env.NODE_ENV === 'development') {
                // APIを模擬するための遅延
                await new Promise(resolve => setTimeout(resolve, 1000));
                
                // キーワードでフィルタリング（開発環境での模擬処理）
                let filteredPapers = mockPapers;
                if (keyword) {
                    filteredPapers = mockPapers.filter(paper => 
                        paper.title.toLowerCase().includes(keyword.toLowerCase()) ||
                        paper.abstract.toLowerCase().includes(keyword.toLowerCase()) ||
                        paper.journal.toLowerCase().includes(keyword.toLowerCase()) ||
                        paper.keywords.some(k => k.toLowerCase().includes(keyword.toLowerCase())) ||
                        paper.primary_category.toLowerCase().includes(keyword.toLowerCase())
                    );
                }
                setPapers(filteredPapers);
            } else {
                // 本番環境では実際のAPIを呼び出し
                const url = new URL('http://localhost:8080/api/v1/academic_paper/all');
                if (keyword) {
                    url.searchParams.append('keyword', keyword);
                }
                
                const response = await fetch(url);
                if (!response.ok) {
                    throw new Error('論文データの取得に失敗しました');
                }
                const results = await response.json();
                const data = results.items || [];
                if (data.length === 0) {
                    setError('検索結果がありません');
                } else {
                    setError(null);
                }
                setPapers(data);
            }
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
            setIsSearching(false);
        }
    };

    const handleSort = (key) => {
        let direction = 'asc';
        if (sortConfig.key === key && sortConfig.direction === 'asc') {
            direction = 'desc';
        }
        setSortConfig({ key, direction });
    };

    const handleViewDetail = (paper_id) => {
        navigate(`/papers/${paper_id}`);
    };

    const handleSearchChange = (e) => {
        const value = e.target.value;
        setSearchKeyword(value);
        setIsSearching(true);
    };

    const handleSearchClear = () => {
        setSearchKeyword('');
        setIsSearching(false);
    };

    const handleKeywordClick = (keyword) => {
        setSearchKeyword(keyword);
        setIsSearching(true);
    };

    const sortedPapers = React.useMemo(() => {
        let sortablePapers = [...papers];
        if (sortConfig.key) {
            sortablePapers.sort((a, b) => {
                if (a[sortConfig.key] < b[sortConfig.key]) {
                    return sortConfig.direction === 'asc' ? -1 : 1;
                }
                if (a[sortConfig.key] > b[sortConfig.key]) {
                    return sortConfig.direction === 'asc' ? 1 : -1;
                }
                return 0;
            });
        }
        return sortablePapers;
    }, [papers, sortConfig]);

    const formatDate = (dateString) => {
        if (!dateString) return '-';
        const date = new Date(dateString);
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    };

    const truncateText = (text, maxLength = 100) => {
        if (!text) return '-';
        return text.length > maxLength ? text.substring(0, maxLength) + '...' : text;
    };

    const renderKeywords = (keywords) => {
        if (!keywords || keywords.length === 0) return '-';
        
        return (
            <div className="keywords-container">
                {keywords.map((keyword, index) => (
                    <span 
                        key={index}
                        className="keyword-tag"
                        onClick={() => handleKeywordClick(keyword)}
                        title={`${keyword}でフィルタする`}
                    >
                        {keyword}
                    </span>
                ))}
            </div>
        );
    };

    const formatPrimaryCategory = (category) => {
        if (!category) return '-';
        return category;
    };

    if (loading && !isSearching) {
        return (
            <div className="academic-paper-table">
                <div className="loading">論文データを読み込み中...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="academic-paper-table">
                <div className="error">エラー: {error}</div>
                <button onClick={() => fetchPapers(searchKeyword)} className="retry-button">
                    再試行
                </button>
            </div>
        );
    }

    return (
        <div className="academic-paper-table">
            <div className="table-header">
                <h2>論文一覧</h2>
                <div className="search-container">
                    <input
                        type="text"
                        placeholder="タイトル、要約、キーワード、カテゴリで検索..."
                        value={searchKeyword}
                        onChange={handleSearchChange}
                        className="search-input"
                    />
                    {searchKeyword && (
                        <button 
                            onClick={handleSearchClear}
                            className="search-clear-button"
                        >
                            ×
                        </button>
                    )}
                    {isSearching && (
                        <div className="search-loading">検索中...</div>
                    )}
                </div>
                <div className="table-actions">
                    <button onClick={() => fetchPapers(searchKeyword)} className="refresh-button">
                        更新
                    </button>
                </div>
            </div>
            
            <div className="table-container">
                <table className="papers-table">
                    <thead>
                        <tr>
                            <th 
                                onClick={() => handleSort('title')}
                                className={sortConfig.key === 'title' ? 'sorted' : ''}
                            >
                                Title
                                {sortConfig.key === 'title' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? '▲' : '▼'}
                                    </span>
                                )}
                            </th>
                            <th 
                                onClick={() => handleSort('keywords')}
                                className={sortConfig.key === 'keywords' ? 'sorted' : ''}
                            >
                                Keywords
                                {sortConfig.key === 'keywords' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? '▲' : '▼'}
                                    </span>
                                )}
                            </th>
                            <th 
                                onClick={() => handleSort('primary_category')}
                                className={sortConfig.key === 'primary_category' ? 'sorted' : ''}
                            >
                                Category
                                {sortConfig.key === 'primary_category' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? '▲' : '▼'}
                                    </span>
                                )}
                            </th>
                            <th 
                                onClick={() => handleSort('published_date')}
                                className={sortConfig.key === 'published_date' ? 'sorted' : ''}
                            >
                                Published Date
                                {sortConfig.key === 'published_date' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? '▲' : '▼'}
                                    </span>
                                )}
                            </th>
                            <th>Details</th>
                        </tr>
                    </thead>
                    <tbody>
                        {sortedPapers.length === 0 ? (
                            <tr>
                                <td colSpan="5" className="no-data">
                                    {searchKeyword ? '検索結果がありません' : '論文データがありません'}
                                </td>
                            </tr>
                        ) : (
                            sortedPapers.map((paper) => (
                                <tr key={paper.paper_id}>
                                    <td className="title-cell">
                                        <a 
                                            href={paper.url} 
                                            target="_blank" 
                                            rel="noopener noreferrer"
                                            className="paper-link"
                                        >
                                            {truncateText(paper.title, 60)}
                                        </a>
                                    </td>
                                    <td className="keywords-cell">
                                        {renderKeywords(paper.keywords)}
                                    </td>
                                    <td className="category-cell">
                                        <span className="category-badge">
                                            {formatPrimaryCategory(paper.primary_category)}
                                        </span>
                                    </td>
                                    <td className="date-cell">
                                        {formatDate(paper.published_date)}
                                    </td>
                                    <td className="actions-cell">
                                        <button 
                                            className="view-button"
                                            onClick={() => handleViewDetail(paper.paper_id)}
                                        >
                                            ➡︎
                                        </button>
                                    </td>
                                </tr>
                            ))
                        )}
                    </tbody>
                </table>
            </div>
            
            <div className="table-footer">
                <span className="paper-count">
                    {searchKeyword ? `検索結果: ${papers.length}件` : `論文数: ${papers.length}件`}
                </span>
                {searchKeyword && (
                    <span className="search-info">
                        キーワード: "{searchKeyword}"
                    </span>
                )}
            </div>
        </div>
    );
};

export default AcademicPaperTable;