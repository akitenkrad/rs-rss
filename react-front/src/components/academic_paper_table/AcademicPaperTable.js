import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import mockPapers from '../../sample_data/academicPaperTables.json';
import './AcademicPaperTable.css';

const AcademicPaperTable = () => {
    const navigate = useNavigate();
    const [papers, setPapers] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
    const [searchKeyword, setSearchKeyword] = useState('');
    const [isSearching, setIsSearching] = useState(false);
    
    // 無限スクロール用の状態を追加
    const [limit, setLimit] = useState(20);
    const [offset, setOffset] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [isLoadingMore, setIsLoadingMore] = useState(false);
    const tableContainerRef = useRef(null);

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

    // スクロールイベントハンドラー
    const handleScroll = useCallback(() => {
        if (!tableContainerRef.current || isLoadingMore || !hasMore) return;

        const { scrollTop, scrollHeight, clientHeight } = tableContainerRef.current;
        
        // スクロール位置が底から100px以内に到達した場合
        if (scrollTop + clientHeight >= scrollHeight - 100) {
            loadMorePapers();
        }
    }, [isLoadingMore, hasMore]);

    // スクロールイベントリスナーの登録
    useEffect(() => {
        const tableContainer = tableContainerRef.current;
        if (tableContainer) {
            tableContainer.addEventListener('scroll', handleScroll);
            return () => {
                tableContainer.removeEventListener('scroll', handleScroll);
            };
        }
    }, [handleScroll]);

    // 追加データの読み込み
    const loadMorePapers = async () => {
        if (isLoadingMore || !hasMore) return;

        try {
            setIsLoadingMore(true);
            
            const nextOffset = offset + limit;
            const newPapers = await fetchPapersData(searchKeyword, nextOffset);
            
            if (newPapers.length === 0) {
                setHasMore(false);
            } else {
                setPapers(prevPapers => [...prevPapers, ...newPapers]);
                setOffset(nextOffset);
            }
        } catch (err) {
            console.error('Error loading more papers:', err);
        } finally {
            setIsLoadingMore(false);
        }
    };

    // データ取得のロジックを分離
    const fetchPapersData = async (keyword = '', currentOffset = 0) => {
        if (process.env.NODE_ENV === 'development') {
            // 開発環境：モックデータを使用
            await new Promise(resolve => setTimeout(resolve, 1000));
            
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
            
            // ページネーションをシミュレート
            const start = currentOffset;
            const end = start + limit;
            return filteredPapers.slice(start, end);
        } else {
            // 本番環境：APIを呼び出し
            const url = new URL('http://localhost:8080/api/v1/academic_paper/all');
            if (keyword) {
                url.searchParams.append('keyword', keyword);
            }
            url.searchParams.append('limit', limit.toString());
            url.searchParams.append('offset', currentOffset.toString());
            
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error('論文データの取得に失敗しました');
            }
            const results = await response.json();
            return results.items || [];
        }
    };

    const fetchPapers = async (keyword = '') => {
        try {
            setLoading(true);
            setError(null);
            setOffset(0);
            setHasMore(true);
            
            const newPapers = await fetchPapersData(keyword, 0);
            
            if (newPapers.length === 0) {
                setError('検索結果がありません');
            } else {
                setError(null);
            }
            setPapers(newPapers);
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
            <div className="academic-paper-table" style={{ marginTop: '80px' }}>
                <div className="loading">論文データを読み込み中...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="academic-paper-table" style={{ marginTop: '80px' }}>
                <div className="error">エラー: {error}</div>
                <button onClick={() => fetchPapers(searchKeyword)} className="retry-button">
                    再試行
                </button>
            </div>
        );
    }

    return (
        <div className="academic-paper-table" style={{ marginTop: '80px' }}>
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
            
            <div 
                className="table-container"
                ref={tableContainerRef}
            >
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
                        {isLoadingMore && (
                            <tr>
                                <td colSpan="5" className="loading-more">
                                    さらに読み込み中...
                                </td>
                            </tr>
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