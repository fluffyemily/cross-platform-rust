/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package com.mozilla.toodle;

import android.content.Context;
import android.support.v7.widget.RecyclerView;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.ViewGroup;
import android.widget.LinearLayout;
import android.widget.TextView;

import com.mozilla.toodle.rust.NativeItemSet;
import com.mozilla.toodle.rust.NativeItemsCallback;
import com.mozilla.toodle.rust.NativeItemsChangedCallback;
import com.mozilla.toodle.rust.ListManager;
import com.mozilla.toodle.rust.Toodle;

import org.jetbrains.annotations.Nullable;

import java.lang.ref.WeakReference;
import java.util.ArrayList;
import java.util.List;

public class ListAdapter extends RecyclerView.Adapter<ListAdapter.ViewHolder> {
    private List<Item> dataset = new ArrayList<>(0);
    private final Context context;

    static class NativeItemsCallbackInner implements NativeItemsChangedCallback {
        private final WeakReference<ListAdapter> listAdapterWeakReference;

        NativeItemsCallbackInner(WeakReference<ListAdapter> listAdapterWeakReference) {
            this.listAdapterWeakReference = listAdapterWeakReference;
        }

        @Override
        public void items() {
            final ListAdapter listAdapter = listAdapterWeakReference.get();
            if (listAdapter == null) {
                return;
            }

            Log.i("RustyToodleJava", "Items changed!");
            try (final Toodle toodle = new Toodle(listAdapter.context)) {
                try (final ListManager listManager = toodle.getListManager()) {
                    listManager.getAllItems(new NativeItemsCallback() {
                        @Override
                        public void items(@Nullable NativeItemSet.ByReference itemSet) {
                            if (itemSet == null) {
                                Log.i("RustyToodleJava", "Got no items!");
                                listAdapter.dataset = new ArrayList<>(0);
                                return;
                            }
                            Log.i("RustyToodleJava", "Got " + itemSet.size() + " items!");
                            listAdapter.dataset = Item.fromNativeItems(itemSet.getItems());
                            listAdapter.notifyDataSetChanged();
                            itemSet.close();
                        }
                    });
                }
            }
        }
    }

    // We must keep reference to the callback around, otherwise it'll get GC'd and the native code
    // will call an empty stub instead of our callback.
    private final NativeItemsChangedCallback nativeItemsChangedCallback = new NativeItemsCallbackInner(
            new WeakReference<>(this)
    );

    ListAdapter(Context context) {
        this.context = context;

        try (final Toodle toodle = new Toodle(context)) {
            try (final ListManager listManager = toodle.getListManager()) {
                listManager.registerChangedItemsCallback(nativeItemsChangedCallback);
            }
        }
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
        ((TextView) holder.itemView.findViewById(R.id.itemTitle)).setText(dataset.get(position).name());
    }

    @Override
    public int getItemCount() {
        return dataset.size();
    }
}
