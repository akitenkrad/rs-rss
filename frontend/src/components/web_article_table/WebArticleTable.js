import Button from '@mui/material/Button';
import Menu from '@mui/material/Menu';
import MenuItem from '@mui/material/MenuItem';
import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Typography from '@mui/material/Typography';
import React, { useCallback, useEffect, useRef, useState } from 'react';
import { webArticles } from '../../sample_data';
import { webArticlesApi } from '../api/Api';
import './WebArticleTable.css';

function WebArticleTable() {
    const [articles, setArticles] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [searchKeyword, setSearchKeyword] = useState('');
    const [isSearching, setIsSearching] = useState(false);
    const [dateFrom, setDateFrom] = useState('');
    const [dateTo, setDateTo] = useState('');
    const [statusFilter, setStatusFilter] = useState(''); // ステータスフィルタを追加
    const [limit] = useState(250);
    const [offset, setOffset] = useState(0);
    const [hasMore, setHasMore] = useState(true);
    const [isLoadingMore, setIsLoadingMore] = useState(false);
    const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
    const [statusMenuAnchor, setStatusMenuAnchor] = useState(null); // ステータスメニューのアンカー
    const [selectedArticleForStatus, setSelectedArticleForStatus] = useState(null); // ステータス変更対象の記事
    const tableContainerRef = useRef(null);

    const fetchArticlesData = useCallback(async (keyword = '', fromDate = '', toDate = '', currentOffset = 0, status = '') => {
        const isDevelopment = process.env.NODE_ENV === 'development';
        
        let filteredArticles = [];
        
        if (isDevelopment) {
            // 開発環境: モックデータを使用
            filteredArticles = [...webArticles];
            
            // キーワードでフィルタリング
            if (keyword) {
                filteredArticles = filteredArticles.filter(article => 
                    article.title.toLowerCase().includes(keyword.toLowerCase()) ||
                    article.summary.toLowerCase().includes(keyword.toLowerCase()) ||
                    (article.site_name && article.site_name.toLowerCase().includes(keyword.toLowerCase()))
                );
            }
            
            // ステータスでフィルタリング
            if (status) {
                filteredArticles = filteredArticles.filter(article => 
                    article.status && article.status.toLowerCase() === status.toLowerCase()
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
            try {
                // フィルタ条件が指定されている場合は getFiltered を使用
                if (keyword || fromDate || toDate || status) {
                    const filterParams = {
                        limit: limit,
                        offset: currentOffset
                    };
                    
                    if (keyword) {
                        filterParams.keyword = keyword;
                    }
                    if (fromDate) {
                        filterParams.start_date = new Date(fromDate).toISOString();
                    }
                    if (toDate) {
                        filterParams.end_date = new Date(toDate).toISOString();
                    }
                    if (status) {
                        filterParams.status = status;
                    }
                    
                    const response = await webArticlesApi.getFiltered(filterParams);
                    filteredArticles = response.items || [];
                } else {
                    // フィルタ条件がない場合は getAll を使用
                    const response = await webArticlesApi.getAll({
                        limit: limit,
                        offset: currentOffset
                    });
                    filteredArticles = response.items || [];
                }
            } catch (error) {
                console.error('Error fetching articles from API:', error);
                throw error;
            }
        }
        
        return filteredArticles;
    }, [limit]);

    const fetchArticles = useCallback(async (keyword = '', fromDate = '', toDate = '', status = '') => {
        try {
            setLoading(true);
            setError(null);
            setOffset(0);
            setHasMore(true);
            
            const newArticles = await fetchArticlesData(keyword, fromDate, toDate, 0, status);
            
            setArticles(newArticles);
            setError(null);
        } catch (err) {
            setError(`記事の取得に失敗しました: ${err.message}`);
            console.error('Error fetching articles:', err);
        } finally {
            setLoading(false);
            setIsSearching(false);
        }
    }, [fetchArticlesData]);

    useEffect(() => {
        fetchArticles();
    }, [fetchArticles]);

    useEffect(() => {
        const debounceTimer = setTimeout(() => {
            fetchArticles(searchKeyword, dateFrom, dateTo, statusFilter);
        }, 500);

        return () => clearTimeout(debounceTimer);
    }, [searchKeyword, dateFrom, dateTo, statusFilter, fetchArticles]);

    // loadMoreArticles関数を先に定義
    const loadMoreArticles = useCallback(async () => {
        if (isLoadingMore || !hasMore) return;

        try {
            setIsLoadingMore(true);
            
            const nextOffset = offset + limit;
            const newArticles = await fetchArticlesData(searchKeyword, dateFrom, dateTo, nextOffset, statusFilter);

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
    }, [isLoadingMore, hasMore, offset, limit, searchKeyword, dateFrom, dateTo, statusFilter, fetchArticlesData]);

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

    const handleStatusFilterChange = (e) => {
        const value = e.target.value;
        setStatusFilter(value);
        setIsSearching(true);
    };

    const handleSearchClear = () => {
        setSearchKeyword('');
        setDateFrom('');
        setDateTo('');
        setStatusFilter('');
        setIsSearching(false);
    };

    const handleArticleRead = async (article, event) => {
        event.preventDefault(); // デフォルトのリンク動作を防ぐ
        
        try {
            // 現在のステータスが 'new' の場合のみ 'archived' に更新
            if (article.status === 'new') {
                // APIを呼び出してステータスを更新（開発環境・本番環境共通）
                await webArticlesApi.updateStatus(article.article_id || article.id, 'archived');
                
                // ローカル状態を更新（記事リストの再描画のため）
                setArticles(prevArticles => 
                    prevArticles.map(art => 
                        (art.article_id || art.id) === (article.article_id || article.id) 
                            ? { ...art, status: 'archived' }
                            : art
                    )
                );
                
                console.log(`Article status updated from 'new' to 'archived' for: ${article.title}`);
            }
        } catch (error) {
            console.error('Failed to update article status:', error);
            // エラーハンドリング（必要に応じてユーザーに通知）
        }
        
        // 記事のページを新しいタブで開く
        window.open(article.url, '_blank', 'noopener,noreferrer');
    };

    // ステータスボタンがクリックされたときの処理
    const handleStatusButtonClick = (event, article) => {
        event.stopPropagation(); // イベントの伝播を停止
        setStatusMenuAnchor(event.currentTarget);
        setSelectedArticleForStatus(article);
    };

    // ステータスメニューを閉じる
    const handleStatusMenuClose = () => {
        setStatusMenuAnchor(null);
        setSelectedArticleForStatus(null);
    };

    // ステータスを変更する
    const handleStatusChange = async (newStatus) => {
        if (!selectedArticleForStatus) return;

        try {
            const articleId = selectedArticleForStatus.article_id || selectedArticleForStatus.id;
            console.log(`Updating article status: ID=${articleId}, newStatus=${newStatus}`);
            
            // APIを呼び出してステータスを更新
            await webArticlesApi.updateStatus(articleId, newStatus);
            
            // ローカル状態を更新
            setArticles(prevArticles => 
                prevArticles.map(art => 
                    (art.article_id || art.id) === articleId 
                        ? { ...art, status: newStatus }
                        : art
                )
            );
            
            console.log(`✅ Article status successfully updated to '${newStatus}' for: ${selectedArticleForStatus.title}`);
        } catch (error) {
            console.error('❌ Failed to update article status:', error);
            // より詳細なエラー情報をログ出力
            console.error('Error details:', {
                articleId: selectedArticleForStatus.article_id || selectedArticleForStatus.id,
                newStatus: newStatus,
                error: error.message
            });
            // エラーハンドリング（必要に応じてユーザーに通知）
            alert(`ステータスの更新に失敗しました: ${error.message}`);
        }
        
        handleStatusMenuClose();
    };

    const formatDate = (dateString) => {
        const date = new Date(dateString);
        return date.toLocaleDateString('ja-JP', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit'
        });
    };

    // ステータスオプションの定義
    const statusOptions = [
        { value: 'new', label: 'NEW', color: 'primary' },
        { value: 'archived', label: 'ARCHIVED', color: 'secondary' }
    ];

    // ステータスに応じたボタンのスタイルを取得する関数
    const getStatusButtonStyle = (status) => {
        const statusLower = status?.toLowerCase();
        
        switch (statusLower) {
            case 'new':
                return {
                    backgroundColor: '#1976d2', // 青色
                    color: 'white',
                    '&:hover': {
                        backgroundColor: '#1565c0'
                    }
                };
            case 'archived':
                return {
                    backgroundColor: '#757575', // 灰色
                    color: 'white',
                    '&:hover': {
                        backgroundColor: '#616161'
                    }
                };
            default:
                return {
                    backgroundColor: '#e0e0e0',
                    color: 'black',
                    '&:hover': {
                        backgroundColor: '#d5d5d5'
                    }
                };
        }
    };

    // ステータスに応じたクラス名を取得する関数を追加
    const getStatusBadgeClass = (status) => {
        if (!status) return '';
        
        const statusLower = status.toLowerCase();
        
        switch (statusLower) {
            case 'new':
            case '新規':
                return 'new';
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
                    <button onClick={() => fetchArticles(searchKeyword, dateFrom, dateTo, statusFilter)} className="retry-button">
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
                        <div className="status-filter-container">
                            <label htmlFor="statusFilter">ステータス:</label>
                            <select
                                id="statusFilter"
                                value={statusFilter}
                                onChange={handleStatusFilterChange}
                                className="status-select"
                            >
                                <option value="">すべて</option>
                                <option value="new">新規</option>
                                <option value="archived">アーカイブ済み</option>
                            </select>
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
                                onClick={() => handleSort('status')}
                                style={{ cursor: 'pointer' }}
                            >
                                Status
                                {sortConfig.key === 'status' && (
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
                                    {searchKeyword || dateFrom || dateTo || statusFilter ? '検索結果がありません' : '記事データがありません'}
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
                                            className={`article-link ${article.status === 'archived' ? 'archived' : ''}`}
                                            onClick={(event) => handleArticleRead(article, event)}
                                        >
                                            {article.status === 'archived' ? '記事を読む (既読)' : '記事を読む'}
                                        </a>
                                    </TableCell>
                                    <TableCell className="status-cell">
                                        <Button
                                            variant="contained"
                                            size="small"
                                            sx={getStatusButtonStyle(article.status)}
                                            onClick={(event) => handleStatusButtonClick(event, article)}
                                        >
                                            {article.status ? article.status.toUpperCase() : '未設定'}
                                        </Button>
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
            
            {/* ステータス変更メニュー */}
            <Menu
                anchorEl={statusMenuAnchor}
                open={Boolean(statusMenuAnchor)}
                onClose={handleStatusMenuClose}
            >
                {statusOptions.map((option) => (
                    <MenuItem 
                        key={option.value} 
                        onClick={() => handleStatusChange(option.value)}
                    >
                        {option.label}
                    </MenuItem>
                ))}
            </Menu>
            
            <div className="table-footer">
                <span className="article-count">
                    {searchKeyword || dateFrom || dateTo || statusFilter ? `検索結果: ${articles.length}件` : `記事数: ${articles.length}件`}
                </span>
                {(searchKeyword || dateFrom || dateTo || statusFilter) && (
                    <span className="search-info">
                        {searchKeyword && (
                            <span className="search-condition">
                                キーワード: <span className="condition-value">"{searchKeyword}"</span>
                            </span>
                        )}
                        {(dateFrom || dateTo) && (
                            <span className="search-condition">
                                期間: <span className="condition-value">{dateFrom || '指定なし'} ～ {dateTo || '指定なし'}</span>
                            </span>
                        )}
                        {statusFilter && (
                            <span className="search-condition">
                                ステータス: <span className="condition-value">{statusFilter === 'new' ? '新規' : statusFilter === 'archived' ? 'アーカイブ済み' : statusFilter}</span>
                            </span>
                        )}
                    </span>
                )}
            </div>
        </div>
    );
}

export default WebArticleTable;