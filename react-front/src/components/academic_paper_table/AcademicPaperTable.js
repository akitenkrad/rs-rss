import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Typography from '@mui/material/Typography';
import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import mockPapers from '../../sample_data/academicPapers.json';
import { academicPapersApi, handleApiError } from '../api/Api';
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
            fetchPapers(searchKeyword);
        }, 500); // 500ms の遅延でAPIを呼び出し

        return () => clearTimeout(debounceTimer);
    }, [searchKeyword]);

    // loadMorePapers関数を先に定義
    const loadMorePapers = useCallback(async () => {
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
    }, [isLoadingMore, hasMore, offset, limit, searchKeyword]);

    // スクロールイベントハンドラー
    const handleScroll = useCallback(() => {
        if (!tableContainerRef.current || isLoadingMore || !hasMore) return;

        const { scrollTop, scrollHeight, clientHeight } = tableContainerRef.current;
        
        // スクロール位置が底から100px以内に到達した場合
        if (scrollTop + clientHeight >= scrollHeight - 100) {
            loadMorePapers();
        }
    }, [isLoadingMore, hasMore, loadMorePapers]);

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
            // 本番環境：APIクライアントを使用
            try {
                const params = {
                    limit: limit.toString(),
                    offset: currentOffset.toString()
                };
                
                if (keyword) {
                    const results = await academicPapersApi.search(keyword, params);
                    return results.items || results || [];
                } else {
                    const results = await academicPapersApi.getAll(params);
                    return results.items || results || [];
                }
            } catch (error) {
                const apiError = handleApiError(error);
                throw new Error(apiError.message || '論文データの取得に失敗しました');
            }
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
                let aValue = a[sortConfig.key];
                let bValue = b[sortConfig.key];
                
                // キーワード配列の場合は最初のキーワードで比較
                if (sortConfig.key === 'keywords') {
                    aValue = (a.keywords && a.keywords.length > 0) ? a.keywords[0] : '';
                    bValue = (b.keywords && b.keywords.length > 0) ? b.keywords[0] : '';
                }
                
                // 日付の場合は Date オブジェクトで比較
                if (sortConfig.key === 'published_date') {
                    aValue = aValue ? new Date(aValue) : new Date(0);
                    bValue = bValue ? new Date(bValue) : new Date(0);
                }
                
                // 文字列の場合は小文字で比較
                if (typeof aValue === 'string') {
                    aValue = aValue.toLowerCase();
                    bValue = bValue.toLowerCase();
                }
                
                if (aValue < bValue) {
                    return sortConfig.direction === 'asc' ? -1 : 1;
                }
                if (aValue > bValue) {
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
                <div className="table-header">
                    <h2>論文一覧</h2>
                </div>
                <div className="loading-message">
                    <Typography>論文データを読み込み中...</Typography>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="academic-paper-table" style={{ marginTop: '80px' }}>
                <div className="table-header">
                    <h2>論文一覧</h2>
                </div>
                <div className="error-message">
                    <Typography color="error">エラー: {error}</Typography>
                    <button onClick={() => fetchPapers(searchKeyword)} className="retry-button">
                        再試行
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="academic-paper-table" style={{ 
            marginTop: '80px',
            height: 'calc(100vh - 80px)',
            display: 'flex',
            flexDirection: 'column'
        }}>
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
            
            <TableContainer 
                component={Paper} 
                className="table-container"
                ref={tableContainerRef}
                sx={{ 
                    flex: 1,
                    height: 'calc(100vh - 200px)', // ヘッダーとフッターを除いた高さ
                    overflow: 'auto',
                    '& .MuiTableHead-root .MuiTableCell-root': {
                        position: 'sticky',
                        top: 0,
                        backgroundColor: '#1a365d',
                        color: '#e2e8f0',
                        borderBottom: '2px solid #2c5282',
                        zIndex: 10
                    }
                }}
            >
                <Table sx={{ minWidth: 650 }} aria-label="academic papers table">
                    <TableHead>
                        <TableRow className="table-header">
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('title')}
                                style={{ cursor: 'pointer' }}
                            >
                                Title
                                {sortConfig.key === 'title' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('keywords')}
                                style={{ cursor: 'pointer' }}
                            >
                                Keywords
                                {sortConfig.key === 'keywords' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('primary_category')}
                                style={{ cursor: 'pointer' }}
                            >
                                Category
                                {sortConfig.key === 'primary_category' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('published_date')}
                                style={{ cursor: 'pointer' }}
                            >
                                Published Date
                                {sortConfig.key === 'published_date' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell className="table-header-cell">Details</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {sortedPapers.length === 0 ? (
                            <TableRow>
                                <TableCell colSpan={5} className="no-data">
                                    {searchKeyword ? '検索結果がありません' : '論文データがありません'}
                                </TableCell>
                            </TableRow>
                        ) : (
                            sortedPapers.map((paper) => (
                                <TableRow key={paper.paper_id} className="table-row">
                                    <TableCell className="title-cell">
                                        <a 
                                            href={paper.url} 
                                            target="_blank" 
                                            rel="noopener noreferrer"
                                            className="paper-link"
                                        >
                                            {truncateText(paper.title, 60)}
                                        </a>
                                    </TableCell>
                                    <TableCell className="keywords-cell">
                                        {renderKeywords(paper.keywords)}
                                    </TableCell>
                                    <TableCell className="category-cell">
                                        <span className="category-badge">
                                            {formatPrimaryCategory(paper.primary_category)}
                                        </span>
                                    </TableCell>
                                    <TableCell className="date-cell">
                                        {formatDate(paper.published_date)}
                                    </TableCell>
                                    <TableCell className="actions-cell">
                                        <button 
                                            className="view-button"
                                            onClick={() => handleViewDetail(paper.paper_id)}
                                        >
                                            ➡︎
                                        </button>
                                    </TableCell>
                                </TableRow>
                            ))
                        )}
                        {isLoadingMore && (
                            <TableRow>
                                <TableCell colSpan={5} className="loading-more">
                                    <Typography>さらに読み込み中...</Typography>
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </TableContainer>
            
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