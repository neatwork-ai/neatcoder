import React from 'react';

type SendButtonProps = {
    onClick: () => void;
    disabled: boolean; // optional prop to disable the button
};

const SendButton: React.FC<SendButtonProps> = ({ onClick, disabled }) => {
    const fillColor = disabled ? 'transparent' : 'var(--vscode-activityBarBadge-background)'; // Choose color based on disabled state

    return (
        <button
            onClick={onClick}
            disabled={disabled} // use the disabled prop here
            style={{
                backgroundColor: 'transparent',
                border: 'none',
                cursor: 'pointer',
                position: 'absolute',
                bottom: '23px',
                right: '25px'
            }}>
            <svg
                style={{display: 'block'}}
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <rect width="24" height="24" rx="2" fill={fillColor} />
                <path d="M5.99637 5.23464C5.91267 5.19277 5.8189 5.17524 5.72573 5.18404C5.63255 5.19283 5.54372 5.22759 5.46933 5.28438C5.39494 5.34116 5.33798 5.41768 5.30493 5.50523C5.27188 5.59279 5.26406 5.68786 5.28237 5.77964L6.68537 10.6296C6.71153 10.72 6.76266 10.8012 6.83289 10.8638C6.90312 10.9264 6.98959 10.968 7.08237 10.9836L12.7724 11.9366C13.0404 11.9896 13.0404 12.3736 12.7724 12.4266L7.08237 13.3796C6.98959 13.3953 6.90312 13.4368 6.83289 13.4995C6.76266 13.5621 6.71153 13.6433 6.68537 13.7336L5.28237 18.5836C5.26406 18.6754 5.27188 18.7705 5.30493 18.858C5.33798 18.9456 5.39494 19.0221 5.46933 19.0789C5.54372 19.1357 5.63255 19.1704 5.72573 19.1792C5.8189 19.188 5.91267 19.1705 5.99637 19.1286L18.9964 12.6286C19.0793 12.5871 19.149 12.5232 19.1978 12.4443C19.2465 12.3654 19.2723 12.2744 19.2723 12.1816C19.2723 12.0889 19.2465 11.9979 19.1978 11.919C19.149 11.84 19.0793 11.7762 18.9964 11.7346L5.99637 5.23464Z" fill="white"/>
            </svg>
        </button>
    );
};

export default SendButton;
