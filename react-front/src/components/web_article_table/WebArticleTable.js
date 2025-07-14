import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Typography from '@mui/material/Typography';
import { useCallback, useEffect, useRef, useState } from 'react';
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
    
            console.log('Loaded articles:', newArticles.length);

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
        console.log('Scroll event triggered:', { scrollTop, scrollHeight, clientHeight, isLoadingMore, hasMore });
        
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
        <div className="web-article-table-container" style={{ marginTop: '80px' }}>
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
            >
                <Table sx={{ minWidth: 650 }} aria-label="web articles table">
                    <TableHead>
                        <TableRow className="table-header">
                            <TableCell className="table-header-cell">日付</TableCell>
                            <TableCell className="table-header-cell">サイト名</TableCell>
                            <TableCell className="table-header-cell">記事タイトル</TableCell>
                            <TableCell className="table-header-cell">要約</TableCell>
                            <TableCell className="table-header-cell">URL</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {articles.length === 0 ? (
                            <TableRow>
                                <TableCell colSpan={5} className="no-data">
                                    {searchKeyword || dateFrom || dateTo ? '検索結果がありません' : '記事データがありません'}
                                </TableCell>
                            </TableRow>
                        ) : (
                            articles.map((article) => (
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