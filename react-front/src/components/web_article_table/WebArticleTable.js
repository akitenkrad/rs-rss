import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Typography from '@mui/material/Typography';
import React, { useCallback, useEffect, useRef, useState } from 'react';
import webArticlesSampleData from '../../sample_data/webArticles.json';
import './WebArticleTable.css';

function WebArticleTable() {
    const [articles, setArticles] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [searchKeyword, setSearchKeyword] = useState('');
    const [isSearching, setIsSearching] = useState(false);
    const [dateFrom, setDateFrom] = useState('');
    const [dateTo, setDateTo] = useState('');
    const [limit, setLimit] = useState(250);
    const [offset, setOffset] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [isLoadingMore, setIsLoadingMore] = useState(false);
    const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
    const tableContainerRef = useRef(null);

    useEffect(() => {
        fetchArticles();
    }, []);

    useEffect(() => {
        const debounceTimer = setTimeout(() => {
            fetchArticles(searchKeyword, dateFrom, dateTo);
        }, 500);

        return () => clearTimeout(debounceTimer);
    }, [searchKeyword, dateFrom, dateTo]);

    // loadMoreArticles関数を先に定義
    const loadMoreArticles = useCallback(async () => {
        if (isLoadingMore || !hasMore) return;

        try {
            setIsLoadingMore(true);
            
            const nextOffset = offset + limit;
            const newArticles = await fetchArticlesData(searchKeyword, dateFrom, dateTo, nextOffset);

            if (newArticles.length === 0) {
                setHasMore(false);
            } else {
                setArticles(prevArticles => [...prevArticles, ...newArticles]);
                setOffset(nextOffset);
            }
        } catch (err) {
            console.error('Error loading more articles:', err);
        } finally {
            setIsLoadingMore(false);
        }
    }, [isLoadingMore, hasMore, offset, limit, searchKeyword, dateFrom, dateTo]);

    const handleScroll = useCallback(() => {
        if (!tableContainerRef.current || isLoadingMore || !hasMore) return;
        const { scrollTop, scrollHeight, clientHeight } = tableContainerRef.current;
        
        if (scrollTop + clientHeight >= scrollHeight - 100) {
            console.log('Loading more articles...');
            loadMoreArticles();
        }
    }, [isLoadingMore, hasMore, loadMoreArticles]);

    useEffect(() => {
        const tableContainer = tableContainerRef.current;
        
        if (tableContainer) {
            console.log('Adding scroll event listener');
            tableContainer.addEventListener('scroll', handleScroll);
            
            return () => {
                console.log('Removing scroll event listener');
                tableContainer.removeEventListener('scroll', handleScroll);
            };
        }
    }, [handleScroll]);

    const fetchArticlesData = async (keyword = '', fromDate = '', toDate = '', currentOffset = 0) => {
        const isDevelopment = process.env.NODE_ENV === 'development';
        
        let filteredArticles = [];
        
        if (isDevelopment) {
            // 開発環境: モックデータを使用
            filteredArticles = [...webArticlesSampleData];
            
            // キーワードでフィルタリング
            if (keyword) {
                filteredArticles = filteredArticles.filter(article => 
                    article.title.toLowerCase().includes(keyword.toLowerCase()) ||
                    article.summary.toLowerCase().includes(keyword.toLowerCase()) ||
                    (article.site_name && article.site_name.toLowerCase().includes(keyword.toLowerCase()))
                );
            }
            
            // 日付でフィルタリング
            if (fromDate || toDate) {
                filteredArticles = filteredArticles.filter(article => {
                    const articleDate = new Date(article.timestamp);
                    const fromDateObj = fromDate ? new Date(fromDate) : null;
                    const toDateObj = toDate ? new Date(toDate) : null;
                    
                    if (fromDateObj) {
                        fromDateObj.setHours(0, 0, 0, 0);
                    }
                    if (toDateObj) {
                        toDateObj.setHours(23, 59, 59, 999);
                    }
                    
                    let isInDateRange = true;
                    
                    if (fromDateObj && articleDate < fromDateObj) {
                        isInDateRange = false;
                    }
                    if (toDateObj && articleDate > toDateObj) {
                        isInDateRange = false;
                    }
                    
                    return isInDateRange;
                });
            }
            
            // ページネーションをシミュレート
            const start = currentOffset;
            const end = start + limit;
            filteredArticles = filteredArticles.slice(start, end);
            
            // 模擬的な遅延
            await new Promise(resolve => setTimeout(resolve, 1000));
        } else {
            // 本番環境: APIからデータを取得
            const apiUrl = `http://localhost:8080/api/v1/web_site/all_web_articles?limit=${limit}&offset=${currentOffset}`;
            const response = await fetch(apiUrl, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            });
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const data = await response.json();
            const allArticles = Array.isArray(data) ? data : data.items || [];
            filteredArticles = allArticles;
        }
        
        return filteredArticles;
    };

    const fetchArticles = async (keyword = '', fromDate = '', toDate = '') => {
        try {
            setLoading(true);
            setError(null);
            setOffset(0);
            setHasMore(true);
            
            const newArticles = await fetchArticlesData(keyword, fromDate, toDate, 0);
            
            setArticles(newArticles);
            setError(null);
        } catch (err) {
            setError(`記事の取得に失敗しました: ${err.message}`);
            console.error('Error fetching articles:', err);
        } finally {
            setLoading(false);
            setIsSearching(false);
        }
    };

    const handleSearchChange = (e) => {
        const value = e.target.value;
        setSearchKeyword(value);
        setIsSearching(true);
    };

    const handleDateFromChange = (e) => {
        const value = e.target.value;
        setDateFrom(value);
        setIsSearching(true);
    };

    const handleDateToChange = (e) => {
        const value = e.target.value;
        setDateTo(value);
        setIsSearching(true);
    };

    const handleSearchClear = () => {
        setSearchKeyword('');
        setDateFrom('');
        setDateTo('');
        setIsSearching(false);
    };

    const formatDate = (dateString) => {
        const date = new Date(dateString);
        return date.toLocaleDateString('ja-JP', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit'
        });
    };

    // ステータスに応じたクラス名を取得する関数を追加
    const getStatusBadgeClass = (status) => {
        if (!status) return '';
        
        const statusLower = status.toLowerCase();
        
        switch (statusLower) {
            case 'active':
            case 'published':
            case '公開':
            case 'アクティブ':
                return 'active';
            case 'inactive':
            case 'draft':
            case '下書き':
            case 'ドラフト':
            case 'todo':
                return 'draft';
            case 'pending':
            case '保留':
            case '承認待ち':
                return 'pending';
            case 'archived':
            case 'アーカイブ':
            case 'done':
            case 'finished':
                return 'archived';
            case 'error':
            case 'エラー':
                return 'error';
            default:
                return '';
        }
    };

    const handleSort = (key) => {
        let direction = 'asc';
        if (sortConfig.key === key && sortConfig.direction === 'asc') {
            direction = 'desc';
        }
        setSortConfig({ key, direction });
    };

    const sortedArticles = React.useMemo(() => {
        let sortableArticles = [...articles];
        if (sortConfig.key) {
            sortableArticles.sort((a, b) => {
                let aValue = a[sortConfig.key];
                let bValue = b[sortConfig.key];
                
                // 日付の場合は Date オブジェクトで比較
                if (sortConfig.key === 'timestamp') {
                    aValue = aValue ? new Date(aValue) : new Date(0);
                    bValue = bValue ? new Date(bValue) : new Date(0);
                }
                
                // 文字列の場合は小文字で比較
                if (typeof aValue === 'string') {
                    aValue = aValue.toLowerCase();
                    bValue = bValue.toLowerCase();
                }
                
                // null/undefined の場合は空文字として扱う
                if (aValue == null) aValue = '';
                if (bValue == null) bValue = '';
                
                if (aValue < bValue) {
                    return sortConfig.direction === 'asc' ? -1 : 1;
                }
                if (aValue > bValue) {
                    return sortConfig.direction === 'asc' ? 1 : -1;
                }
                return 0;
            });
        }
        return sortableArticles;
    }, [articles, sortConfig]);

    if (loading && !isSearching) {
        return (
            <div className="web-article-table-container" style={{ marginTop: '80px' }}>
                <div className="table-header">
                    <h2>Web記事一覧</h2>
                </div>
                <div className="loading-message">
                    <Typography>記事データを読み込み中...</Typography>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="web-article-table-container" style={{ marginTop: '80px' }}>
                <div className="table-header">
                    <h2>Web記事一覧</h2>
                </div>
                <div className="error-message">
                    <Typography color="error">エラー: {error}</Typography>
                    <button onClick={() => fetchArticles(searchKeyword, dateFrom, dateTo)} className="retry-button">
                        再試行
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="web-article-table-container" style={{ 
            marginTop: '80px',
            height: 'calc(100vh - 80px)',
            display: 'flex',
            flexDirection: 'column'
        }}>
            <div className="table-header">
                <h2>Web記事一覧</h2>
                <div className="filters-container">
                    <div className="search-container">
                        <input
                            type="text"
                            placeholder="タイトル、要約、サイト名で検索..."
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
                    <div className="date-filters">
                        <div className="date-input-container">
                            <label htmlFor="dateFrom">開始日:</label>
                            <input
                                type="date"
                                id="dateFrom"
                                value={dateFrom}
                                onChange={handleDateFromChange}
                                className="date-input"
                            />
                        </div>
                        <div className="date-input-container">
                            <label htmlFor="dateTo">終了日:</label>
                            <input
                                type="date"
                                id="dateTo"
                                value={dateTo}
                                onChange={handleDateToChange}
                                className="date-input"
                            />
                        </div>
                    </div>
                </div>
                <div className="table-actions">
                    <button onClick={handleSearchClear} className="clear-button">
                        検索条件をクリア
                    </button>
                </div>
            </div>
            
            <TableContainer 
                component={Paper} 
                className="table-container"
                ref={tableContainerRef}
                sx={{ 
                    flex: 1,
                    height: 'calc(100vh - 200px)',
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
                <Table sx={{ minWidth: 650 }} aria-label="web articles table">
                    <TableHead>
                        <TableRow className="table-header">
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('timestamp')}
                                style={{ cursor: 'pointer' }}
                            >
                                日付
                                {sortConfig.key === 'timestamp' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('site_name')}
                                style={{ cursor: 'pointer' }}
                            >
                                サイト名
                                {sortConfig.key === 'site_name' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('title')}
                                style={{ cursor: 'pointer' }}
                            >
                                記事タイトル
                                {sortConfig.key === 'title' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('summary')}
                                style={{ cursor: 'pointer' }}
                            >
                                要約
                                {sortConfig.key === 'summary' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                            <TableCell className="table-header-cell">URL</TableCell>
                            <TableCell 
                                className="table-header-cell sortable-header"
                                onClick={() => handleSort('status_name')}
                                style={{ cursor: 'pointer' }}
                            >
                                Status
                                {sortConfig.key === 'status_name' && (
                                    <span className="sort-indicator">
                                        {sortConfig.direction === 'asc' ? ' ▲' : ' ▼'}
                                    </span>
                                )}
                            </TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {sortedArticles.length === 0 ? (
                            <TableRow>
                                <TableCell colSpan={6} className="no-data">
                                    {searchKeyword || dateFrom || dateTo ? '検索結果がありません' : '記事データがありません'}
                                </TableCell>
                            </TableRow>
                        ) : (
                            sortedArticles.map((article) => (
                                <TableRow key={article.id} className="table-row">
                                    <TableCell className="date-cell">
                                        {formatDate(article.timestamp || '')}
                                    </TableCell>
                                    <TableCell className="site-name-cell">
                                        <a href={article.site_url || ''} target="_blank" rel="noopener noreferrer" className="article-link">
                                            {article.site_name || ''}
                                        </a>
                                    </TableCell>
                                    <TableCell className="title-cell">
                                        <Typography variant="body1" className="article-title">
                                            {article.title}
                                        </Typography>
                                    </TableCell>
                                    <TableCell className="summary-cell">
                                        <Typography variant="body2" className="article-summary">
                                            {article.summary}
                                        </Typography>
                                    </TableCell>
                                    <TableCell className="url-cell">
                                        <a 
                                            href={article.url} 
                                            target="_blank" 
                                            rel="noopener noreferrer"
                                            className="article-link"
                                        >
                                            記事を読む
                                        </a>
                                    </TableCell>
                                    <TableCell className="status-cell">
                                        <span className={`status-badge ${getStatusBadgeClass(article.status_name)}`}>
                                            {article.status_name || '未設定'}
                                        </span>
                                    </TableCell>
                                </TableRow>
                            ))
                        )}
                        {isLoadingMore && (
                            <TableRow>
                                <TableCell colSpan={6} className="loading-more">
                                    <Typography>さらに読み込み中...</Typography>
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </TableContainer>
            
            <div className="table-footer">
                <span className="article-count">
                    {searchKeyword || dateFrom || dateTo ? `検索結果: ${articles.length}件` : `記事数: ${articles.length}件`}
                </span>
                {(searchKeyword || dateFrom || dateTo) && (
                    <span className="search-info">
                        {searchKeyword && `キーワード: "${searchKeyword}"`}
                        {(searchKeyword && (dateFrom || dateTo)) && ' | '}
                        {(dateFrom || dateTo) && `期間: ${dateFrom || '指定なし'} ～ ${dateTo || '指定なし'}`}
                    </span>
                )}
            </div>
        </div>
    );
}

export default WebArticleTable;