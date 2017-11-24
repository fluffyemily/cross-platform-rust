package com.mozilla.toodle;

import android.support.v7.widget.RecyclerView;
import android.view.LayoutInflater;
import android.view.ViewGroup;
import android.widget.LinearLayout;
import android.widget.TextView;

public class ListAdapter extends RecyclerView.Adapter<ListAdapter.ViewHolder> {
    private final String[] dataset;

    ListAdapter(String[] dataset) {
        this.dataset = dataset;
    }

    static class ViewHolder extends RecyclerView.ViewHolder {
        private final LinearLayout itemView;

        ViewHolder(LinearLayout v) {
            super(v);
            itemView = v;
        }
    }

    @Override
    public ViewHolder onCreateViewHolder(ViewGroup parent, int viewType) {
        final LinearLayout v = (LinearLayout) LayoutInflater.from(parent.getContext())
                .inflate(R.layout.item, parent, false);

        return new ViewHolder(v);

    }

    @Override
    public void onBindViewHolder(ViewHolder holder, int position) {
        ((TextView) holder.itemView.findViewById(R.id.item_title)).setText(dataset[position]);
    }

    @Override
    public int getItemCount() {
        return dataset.length;
    }
}
